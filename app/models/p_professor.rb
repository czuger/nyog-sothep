class PProfessor < ApplicationRecord
  belongs_to :current_location, polymorphic: true

  def loose_life( game_board, investigator, amount )
    decrement!( :hp, amount )
    EEventLog.log( game_board, I18n.t( 'prof_fight.gun_shot', investigator_name: investigator.translated_name,
                   hit: amount, final_hp: hp ) )

    # TODO : implement gamover on professor death
  end

end
