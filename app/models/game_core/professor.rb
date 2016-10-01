module GameCore
  module Professor

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

    def professor_pick_one_monster
      try_counter = 0
      begin
        dice_rolled = GameCore::Dices.d8( 2 )
        monster_choosed = MONSTERS_ROLL_MAP[ dice_rolled ]
        raise "#{dice_rolled} correspond to no monster" unless monster_choosed

        monster = m_monsters.find_by_code_name( monster_choosed )
        try_counter += 1
      end until monster || try_counter > 10
      if monster
        p_monsters.create!( code_name: monster.code_name )
        monster.destroy!
        EEventLog.log( self, I18n.t( 'actions.result.prof_pick_monsters' ) )
      else
        EEventLog.log( self, I18n.t( 'errors.no_more_monsters' ) )
      end
    end

    def professor_pick_start_monsters
      1.upto( 4 ).each do
        professor_pick_one_monster
      end
    end

  end
end

