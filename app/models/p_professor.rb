class PProfessor < ApplicationRecord

  belongs_to :g_game_board
  belongs_to :current_location, polymorphic: true

  include GameCore::ProfessorActions

  def prof_has_to_choose_attack?
    prof_loc = current_location
    gb = g_game_board

    if prof_loc.city?
      if ( inv_to_attack = gb.i_investigators.where( current_location_id: prof_loc.id ).order( :id ).first )
        if GameCore::Dices.d6 <= 2
          prof_fight( inv_to_attack )
        else
          prof_choose_attack = true
        end
      else
        gb.prof_breed!
      end
    else
      gb.prof_breed!
    end
    [ prof_choose_attack, inv_to_attack ]
  end

  def can_breed_in_city?( position )
    !g_game_board.p_monster_positions.exists?( location_id: position.id )
  end

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
      EEventLog.log( gb, I18n.t( 'actions.result.prof_pick_monsters' ) )
    else
      EEventLog.log( gb, I18n.t( 'errors.no_more_monsters' ) )
    end
  end

  private

  def loose_life( gb, investigator, amount )
    decrement!( :hp, amount )
    EEventLog.log( gb, I18n.t( 'prof_fight.gun_shot', investigator_name: investigator.translated_name,
                               hit: amount, final_hp: hp ) )

    # TODO : implement gamover on professor death
  end

  def assert_breed_validity( position )
    raise "Can only breed in city : #{position.inspect}" unless position.city?
    raise "Can't breed in occupied city : #{position.inspect}" unless can_breed_in_city?( position )
    raise "Deep one can only be breed in port : #{position.inspect}" unless position.city?
  end

end
