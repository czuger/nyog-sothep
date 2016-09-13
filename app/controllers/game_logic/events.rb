module GameLogic::Events

  include GameLogic::Dices

  def roll_event( investigator )
    if investigator.current_location.city? && investigator.last_location.city?

      roll = d6
      roll += d6 if investigator.sign
      roll += d6 if investigator.sign && investigator.medaillon

      if investigator.event_table == 1
        GameCore::EventGroundA.send( "table#{investigator.event_table}_e#{roll}", @game_board, investigator, @professor )
      elsif investigator.event_table == 2
        GameCore::EventGroundA.send( "table#{investigator.event_table}_e#{roll}", @game_board, investigator, @professor )
      end
    end
    # If nothing else happens, then the investigator is ready for next adventures
    investigator.be_ready! if investigator.move_done?
  end

end