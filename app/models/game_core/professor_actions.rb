module GameCore
  # Included in PProfessor
  module ProfessorActions

    include GameCore::Assertions

    MONSTERS_ROLL_MAP = {
      2 => :goules,
      3 => :goules,
      4 => :profonds,
      5 => :choses_brume,
      6 => :habitants,
      7 => :reves,
      8 => :reves,
      9 => :reves,
      10 => :fanatiques,
      11 => :fanatiques,
      12 => :fanatiques }

    # Professor pick monsters
    def pick_one_monster
      gb = g_game_board

      try_counter = 0
      begin
        dice_rolled = GameCore::Dices.d6( 2 )
        monster_choosed = MONSTERS_ROLL_MAP[ dice_rolled ]
        raise "#{dice_rolled} correspond to no monster" unless monster_choosed

        monster = gb.m_monsters.find_by_code_name( monster_choosed )
        try_counter += 1
      end until monster || try_counter > 10
      if monster
        gb.p_monsters.create!( code_name: monster.code_name )
        monster.destroy!
        # EEventLog.log( gb, I18n.t( 'actions.result.prof_pick_monsters' ) )
      else
        # EEventLog.log( gb, I18n.t( 'errors.no_more_monsters' ) )
      end
    end

    def breed( monster )

      gb = g_game_board

      # raise "Prof breed called while g_game_board not in prof_breed state : #{gb.inspect}" unless gb.prof_breed?

      ActiveRecord::Base.transaction do
        monster_loc = GameCore::Map::Location.get_location(gb.p_professor.current_location_code_name )

        assert_breed_validity( monster_loc )

        gb.p_monster_positions.create!( location_code_name: monster_loc.code_name, code_name: monster.code_name, token_rotation: rand(-15 .. 15 ) )
        monster.destroy!
        # gb.inv_move!
        #
        # EEventLog.start_event_block( gb )
        # gb.p_professor.pick_one_monster

        # TODO : need to check that prof has no more than 5 monsters.
        # Need to ask him to remove one if this is the case
      end
    end

    def check_nyog_sothep_invocation
      gb = g_game_board

      unless gb.nyog_sothep_invoked
        nb_fanatiques = gb.p_monster_positions.where( code_name: 'fanatiques' ).count()
        if nb_fanatiques >= 5

          return true if current_location_code_name == gb.nyog_sothep_invocation_position_code_name

          _, dist_to_nyog = GameCore::Ia::BfsAlgo.find_next_dest_to_goal(
            current_location_code_name, gb.nyog_sothep_invocation_position_code_name )
          return true if dist_to_nyog <= 3
        end
      end

      false
    end

    def move( dest_loc )

      gb = g_game_board
      prof = gb.p_professor

      assert_regular_movement_allowed( prof.current_location, dest_loc )
      ActiveRecord::Base.transaction do

        if move_nyog_sothep( gb, prof, dest_loc )
          prof.current_location_code_name = dest_loc.code_name
          prof.save!
        end

      end
    end

    # Return true if nyog sothep not invoked, nyog sothep not with prof or nyog sothep allowed to move
    def move_nyog_sothep( gb, prof, dest_loc )
      if gb.nyog_sothep_invoked
        raise "Nyog Sothep current location should not be nil" unless gb.nyog_sothep_current_location_code_name

        if gb.nyog_sothep_current_location_code_name == prof.current_location_code_name

          if rand( 1 .. 6 ) <= 2
            # Nyog sothep and the professor are forbidden to move
            event_log = EEventLog.log( gb, prof, I18n.t( 'map.event_log_summaries.nyog_cant_move' ) )
            EEventLogSummary.log( gb, prof, :nyog_cant_move, {}, event_log )
            return false
          end

          gb.nyog_sothep_current_location_code_name = dest_loc.code_name
          gb.save!

          GDestroyedCity.destroy_city( gb, dest_loc.code_name )
        end
      end
      true
    end
  end
end

