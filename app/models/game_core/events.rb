module GameCore
  class Events

    def self.roll_event( game_board, investigator, professor )
      if investigator.current_location.city? && investigator.last_location.city?

        roll = event_dices( investigator )

        if investigator.event_table == 1
          GameCore::EventGroundA.send( "table#{investigator.event_table}_e#{roll}", game_board, investigator, professor )
        elsif investigator.event_table == 2
          GameCore::EventGroundB.send( "table#{investigator.event_table}_e#{roll}", game_board, investigator, professor )
        else
          raise "Bad event table for investigator : #{investigator.inspect}"
        end
      else
        EEventLog.log( game_board, I18n.t( 'events.no_water_events' ) )
      end
    end

    def self.event_dices( investigator )
      roll = GameCore::Dices.d6
      roll += GameCore::Dices.d6 if investigator.sign
      roll += GameCore::Dices.d6 if investigator.sign && investigator.medaillon
      roll
    end
  end

end
