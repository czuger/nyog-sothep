module GameLogic::Events

  include GameLogic::Dices
  include GameLogic::EventGroundA
  include GameLogic::EventGroundB

  def roll_event
    if @current_investigator.current_location.city? && @last_location.city?

      roll = d6
      roll += d6 if @current_investigator.sign
      roll += d6 if @current_investigator.sign && @current_investigator.medaillon

      if @current_investigator.event_table == 1
        send( "table#{@current_investigator.event_table}_e#{roll}", @current_investigator )
      elsif @current_investigator.event_table == 2
        send( "table#{@current_investigator.event_table}_e#{roll}", @current_investigator )
      end
    end
  end

  def method_missing( method_name, _ )
    super unless method_name =~ /table.*/
    EEventLog.log( "Evenement non implémenté : #{method_name}" )
  end

end