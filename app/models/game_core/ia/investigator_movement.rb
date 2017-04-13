module GameCore
  module Ia
    module InvestigatorMovement

      include GameCore::Movement

      def ia_invest_random_move( game_board )

        zone = current_location
        aval_destinations = zone.destinations

        EEventLog.start_event_block( game_board )

        regular_move_token( game_board, self, aval_destinations.sample )

      end
    end
  end
end