module GameCore
  module Ia
    # Included in investigator
    module InvestigatorActions

      def ia_actions( game_board, prof, prof_position_finder )

        # We resolve an encounter only if investigator is in event mode
        # IE if not at psy or dead
        game_board.resolve_encounter( self ) if events?

        # If he is still in events (if he is not dead)
        if events?

          # Once nyog sothep invoked, we skip events.
          ia_play_events( game_board, prof, prof_position_finder ) unless game_board.nyog_sothep_invoked

          # Break out of the investigators loop
          if game_board.prof_asked_for_fake_cities?
            # If investigator is still alive
            events_done! if events?
            return :break
          else
            # When the turn of the investigator is finished we need to check for a prof fight
            check_for_prof_to_fight_in_city( game_board, prof, prof_position_finder )
            events_done! if events?
          end
        end
      end

      def go_to_psy(game_board )

        san = GameCore::Dices.d6
        self.increment!( :san, san )
        going_to_psy!

        LLog.log( game_board, self, 'actions.go_psy', true,
                  { san_gain: san, cur_san: self.san }, true )
      end

      private

      def ia_play_events( game_board, prof, prof_position_finder )
        # Investigator event check
        roll_event( game_board, prof, prof_position_finder )
      end

    end
  end
end
