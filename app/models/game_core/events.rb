# To be included in investigator
module GameCore
  module Events

    include GameCore::EventGroundA
    include GameCore::EventGroundB

    def roll_event( game_board, professor )
      if current_location.city? && last_location.city?

        table = choose_table
        roll = event_dices

        #  For dev
        # table = 1
        # roll = 18

        # EEventLog.log( game_board, self, "table#{table}_e#{roll}" )

        send( "table#{table}_e#{roll}", game_board, professor )
      else
        EEventLog.log( game_board, self, I18n.t( 'events.no_water_events', investigator_name: translated_name ) )
      end
    end

    def method_missing( method_name, *args )
      super unless method_name =~ /table.*/
      EEventLog.log( args.first, self, "event non implementé : #{method_name}" )
    end

    def choose_table
      Kernel.rand( 1 ..2 )
    end

    def event_dices
      roll = GameCore::Dices.d6
      roll += GameCore::Dices.d6 if sign
      roll += GameCore::Dices.d6 if sign && medaillon
      roll
    end

    private

    def ask_prof_for_fake_cities( game_board, nb_cities )
      game_board.update( asked_fake_cities_count: nb_cities, asked_fake_cities_investigator: self )
      game_board.ask_prof_for_fake_cities!
    end

    def log_event( game_board, method )
      EEventLog.log( game_board, self, ( I18n.t( "events.#{method.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
    end

  end

end
