module GameLogic::Events

  include GameLogic::Dices

  def roll_event
    if @current_investigator.current_location.city? && @last_location.city?

      roll = d6
      roll += d6 if @current_investigator.sign
      roll += d6 if @current_investigator.sign && @current_investigator.medaillon

      if @current_investigator.event_table == 1
        GameCore::EventGroundA.send( "table#{@current_investigator.event_table}_e#{roll}", @game_board, @current_investigator, @professor, @last_location )
      elsif @current_investigator.event_table == 2
        GameCore::EventGroundA.send( "table#{@current_investigator.event_table}_e#{roll}", @game_board, @current_investigator, @professor, @last_location )
      end
    end
  end

end