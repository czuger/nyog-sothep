class PProfessor < ApplicationRecord

  belongs_to :g_game_board
  belongs_to :current_location, polymorphic: true

  include GameCore::ProfessorActions

  def can_breed_in_city?( position )
    !g_game_board.p_monster_positions.exists?( location_id: position.id )
  end

  private

  def loose_life( gb, investigator, amount )
    decrement!( :hp, amount )
    EEventLog.log( gb, investigator,I18n.t( 'prof_fight.gun_shot', investigator_name: investigator.translated_name,
                               hit: amount, final_hp: hp ) )

    # TODO : implement gamover on professor death
  end

  def assert_breed_validity( position )
    raise "Can only breed in city : #{position.inspect}" unless position.city?
    raise "Can't breed in occupied city : #{position.inspect}" unless can_breed_in_city?( position )
    raise "Deep one can only be breed in port : #{position.inspect}" unless position.city?
  end

end
