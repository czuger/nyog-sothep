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


    def prof_play( game_board, dest_loc )

      # If we have a destination, then the prof move, otherwise it stay where he is
      # Nyog sothep movement is tested in the method
      move_to_dest_loc( dest_loc ) if dest_loc

      # If nyog sothep is with the prof, then we check for fight
      game_board.nyog_sothep_repelled_test
      return if game_board.game_lost? || game_board.game_won?

      # We check for investigators to fight in the city.
      check_for_investigators_to_fight_in_city( game_board )
      return if game_board.game_lost? || game_board.game_won?

      # If the prof has less than 5 monsters, he pick one.
      if game_board.p_monsters.count < 5
        pick_one_monster
      end

      # The prof movement is done, we will skip to investigators IA phase.
      game_board.prof_movement_done!

      # At the end of the professor turn, investigators play
      game_board.investigators_ia_play( self )
      game_board.save!

    end


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
        # LLog.log( gb, I18n.t( 'actions.result.prof_pick_monsters' ) )
      else
        # LLog.log( gb, I18n.t( 'errors.no_more_monsters' ) )
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
        # LLog.start_event_block( gb )
        # gb.p_professor.pick_one_monster

        # TODO : need to check that prof has no more than 5 monsters.
        # Need to ask him to remove one if this is the case
      end
    end

    def move_to_dest_loc( dest_loc )

      gb = g_game_board
      prof = gb.p_professor

      assert_regular_movement_allowed( prof.current_location, dest_loc )
      ActiveRecord::Base.transaction do

        if gb.move_nyog_sothep( prof, dest_loc )
          prof.current_location_code_name = dest_loc.code_name
          prof.save!
        end

      end
    end

  end
end

