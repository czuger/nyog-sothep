# To be included in investigator
module GameCore
  module Events

    include GameCore::EventGroundA
    include GameCore::EventGroundB

    def roll_event( game_board, professor )
      if current_location.city? && last_location.city?

        table = rand( 1 ..2 )
        roll = event_dices

        #  For dev
        # table = 1
        # roll = 18

        EEventLog.log( game_board, self, "table#{table}_e#{roll}" )

        send( "table#{table}_e#{roll}", game_board, professor )
      else
        EEventLog.log( game_board, self, I18n.t( 'events.no_water_events' ) )
      end
    end

    def method_missing( method_name, *args )
      super unless method_name =~ /table.*/
      EEventLog.log( args.first, self, "event non implementé : #{method_name}" )
    end

    private

    def event_dices
      roll = GameCore::Dices.d6
      roll += GameCore::Dices.d6 if sign
      roll += GameCore::Dices.d6 if sign && medaillon
      roll
    end

    def ask_prof_for_fake_cities( game_board, nb_cities )
      game_board.update( asked_fake_cities_count: nb_cities, asked_fake_cities_investigator: self )
      game_board.ask_prof_for_fake_cities!
    end

    def prof_spotted( game_board, professor )
      # If the prof is really spotted we create 5 records to increase the probability of targeting
      ActiveRecord::Base.transaction do
        city = professor.current_location
        1.upto( 5 ).each do
          IInvTargetPosition.create!( g_game_board_id: game_board.id, position: city, memory_counter: 5 )
        end
      end

    end

  end

end
