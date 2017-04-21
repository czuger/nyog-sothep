module GameCore
  # Included in PProfessor
  module ProfessorActions

    include GameCore::Movement

    MONSTERS_ROLL_MAP = {
      3 => :horreur_volante,
      2 => :goules,
      4 => :goules,
      5 => :profonds,
      6 => :profonds,
      7 => :chose_brume,
      8 => :habitants,
      9 => :reves,
      10 => :reves,
      11 => :reves,
      12 => :teleportation,
      13 => :fanatiques,
      14 => :tempete,
      15 => :fanatiques,
      16 => :fanatiques }

    # Professor pick monsters
    def pick_one_monster
      gb = g_game_board

      try_counter = 0
      begin
        dice_rolled = GameCore::Dices.d8( 2 )
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
        monster_loc = gb.p_professor.current_location

        assert_breed_validity( monster_loc )

        gb.p_monster_positions.create!( location: monster_loc, code_name: monster.code_name, token_rotation: rand(-15 .. 15 ) )
        monster.destroy!
        # gb.inv_move!
        #
        # EEventLog.start_event_block( gb )
        # gb.p_professor.pick_one_monster

        # TODO : need to check that prof has no more than 5 monsters.
        # Need to ask him to remove one if this is the case
      end
    end

    def move( dest_loc )

      gb = g_game_board
      
      ActiveRecord::Base.transaction do
        regular_move_token( gb, gb.p_professor, dest_loc )

        # gb.prof_attack! if gb.prof_move?
        # gb.prof_attack_after_fall_back! if gb.prof_fall_back?

      end
    end

    def prof_fight( investigator )

      # TODO : check that investigator and prof are at the same place
      # raise "Investigator and prof not at the same place : #{investigator}"

      gb = g_game_board

      ActiveRecord::Base.transaction do

        if investigator.weapon
          result = GameCore::Dices.d6
          case result
            when 6
              loose_life( gb, investigator, 3 )
            when 2..5
              loose_life( gb, investigator, 2 )
            when 1
              EEventLog.log( gb, investigator, I18n.t( 'prof_fight.gun_miss', investigator_name: investigator.translated_name ) )
            else
              raise "Quantic perturbation : roll = #{result}"
          end
          # gb.prof_fall_back!

        elsif investigator.sign
          EEventLog.log( gb, investigator, I18n.t( 'prof_fight.sign_protect', investigator_name: investigator.translated_name ) )
          investigator.loose_san( gb, 2 )
          # gb.inv_repelled!

        else
          EEventLog.log( gb, investigator, I18n.t( 'prof_fight.no_protection', investigator_name: investigator.translated_name ) )
          investigator.loose_san( gb, 4 )
          # gb.inv_repelled!

        end
      end

    end

  end
end

