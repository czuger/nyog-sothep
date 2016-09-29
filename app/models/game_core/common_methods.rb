module GameCore
  module CommonMethods

    def investigator_goes_back( game_board, investigator )
      ll = investigator.last_location
      investigator.current_location = ll
      investigator.save!
      EEventLog.log_investigator_movement( game_board, investigator, ll, direction: :return )
    end

    def investigator_loose_san( game_board, investigator, san_amount )
      investigator.decrement!( :san, san_amount )
      EEventLog.log( game_board, I18n.t( 'actions.result.perte_san', san: san_amount,
        investigator_name: investigator.translated_name, final_san: investigator.san ) )
      if investigator.san <= 0
        investigator_die( game_board, investigator )
        return false
      end
      true
    end

    def investigator_die( game_board, investigator )
      EEventLog.log( game_board, I18n.t( "actions.result.crazy.#{investigator.gender}", investigator_name: investigator.translated_name ) )
      game_board.p_monster_positions.create!(
        location: investigator.current_location, code_name: 'fanatiques', discovered: true )
      investigator.update_attribute( :dead, true )
    end

  end
end
