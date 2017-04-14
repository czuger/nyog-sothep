# To be included in investigator
module GameCore
  module Events

    include GameCore::EventGroundA
    include GameCore::EventGroundB

    def roll_event( game_board, professor )
      if current_location.city? && last_location.city?

        table = rand( 1 ..2 )
        roll = event_dices

        send( "table#{table}_e#{roll}", game_board, professor )
      else
        EEventLog.log( game_board, I18n.t( 'events.no_water_events' ) )
      end
    end

    def event_dices
      roll = GameCore::Dices.d6
      roll += GameCore::Dices.d6 if sign
      roll += GameCore::Dices.d6 if sign && medaillon
      roll
    end

    def method_missing( method_name, *args )
      super unless method_name =~ /table.*/
      EEventLog.log( args.first, 'event non implementé' )
    end

  end

end
