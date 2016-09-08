module GameLogic::Events

  def roll_event
    GameLogicM::Event.send( "e#{ rand( 1..6 ) }", @current_investigator )
  end

end