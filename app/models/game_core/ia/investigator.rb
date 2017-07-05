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

        LLog.log( game_board, self, 'log.go_psy', { san_gain: san, cur_san: self.san }, true )

      end
    end
  end
end