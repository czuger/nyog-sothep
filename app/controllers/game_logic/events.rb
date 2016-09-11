module GameLogic::Events

  include GameLogic::Dices

  def roll_event
    roll = d6
    roll += d6 if @current_investigator.sign
    roll += d6 if @current_investigator.sign && @current_investigator.medaillon
    if @current_investigator.current_location.class == CCity
      if @current_investigator.event_table == 1
        GameLogicM::EventGroundA.send( "e#{roll}", @current_investigator )
      else
        GameLogicM::EventGroundB.send( "e#{roll}", @current_investigator )
      end
    end
  end

end