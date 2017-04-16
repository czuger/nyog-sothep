module GameCore
  module Ia
    module Investigator

      include GameCore::Ia::InvestigatorMovement

      def ia_play( game_board, prof )
        if san < 5 && current_location.city?
          # If san is below 5 and investigator is in city, then he/she goes to the psy
          go_psy( game_board )
        else
          ia_invest_random_move( game_board )

          # Investigator encounters check

          game_board.resolve_encounter( self )

          # Investigator event check
          roll_event( game_board, prof )
        end

        update( current: false )
      end

      private

      def go_psy( game_board )
        EEventLog.start_event_block( game_board )
        san = GameCore::Dices.d6
        self.increment!( :san, san )
        # EEventLog.log( game_board, I18n.t( "actions.psy.#{gender}", san: san,
        #                                    investigator_name: I18n.t( "investigators.#{code_name}" ) ) )
        EEventLog.log( game_board, I18n.t( 'actions.psy', san: san, investigator_name: I18n.t( "investigators.#{code_name}" ) ) )
      end
    end
  end
end