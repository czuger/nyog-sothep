class PProfessor < ApplicationRecord

  belongs_to :g_game_board

  include GameCore::ProfessorActions
  include GameCore::ProfessorFight

  def gender
    'm'
  end

  def can_breed_in_city?( position_code_name )
    !g_game_board.p_monster_positions.exists?( location_code_name: position_code_name )
  end

  def spotted( game_board, prof_position_finder )
    city = current_location
    # Le prof n'est jamais repéré dans l'eau
    unless city.water_area?
      prof_position_finder.spot_prof( game_board.turn, prof_position_finder )
    end
  end

  def current_location
    GameCore::Map::Location.get_location( current_location_code_name )
  end

  def translated_name
    'Le professeur'
  end

  private

  def loose_life( gb, investigator, amount )
    decrement!( :hp, amount )
    LLog.log( gb, investigator,'fight.gun_shot', true, { life_loss: amount, cur_life: hp } )

    # TODO : implement gamover on professor death
  end

  def assert_breed_validity( position )
    raise "Can only breed in city : #{position.inspect}" unless position.city?
    raise "Can't breed in occupied city : #{position.inspect}" unless can_breed_in_city?( position.code_name )
    raise "Deep one can only be breed in port : #{position.inspect}" unless position.city?
  end

end
