module GameCore
  module Ia
    # Included in investigator
    module Investigator

      include GameCore::Ia::InvestigatorMovement
      include GameCore::Ia::InvestigatorActions

      private

      def ia_play_events( game_board, prof )
        # Investigator event check
        roll_event( game_board, prof )
      end

      def go_to_psy(game_board )

        san = GameCore::Dices.d6
        self.increment!( :san, san )
        going_to_psy!

        # EEventLog.log( game_board, I18n.t( "actions.psy.#{gender}", san: san,
        #                                    investigator_name: I18n.t( "investigators.#{code_name}

        EEventLog.log( game_board, self, I18n.t( 'actions.psy', san: san, investigator_name: I18n.t( "investigators.#{code_name}" ) ) )
      end
    end
  end
end