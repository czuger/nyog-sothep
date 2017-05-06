module GameCore
  module Ia
    # Included in investigator
    module InvestigatorActions

      def ia_actions( game_board, prof )

        if skip_turns && skip_turns > 0
          decrement!( :skip_turns )
          if skip_turns <= 0
            gain_san( game_board, san_gain_after_lost_turns ) if san_gain_after_lost_turns
          end
          movement_done!
          events_done!
        else
          # We resolve an encounter only if investigator is in event mode
          # IE if not at psy or dead
          game_board.resolve_encounter( self ) if events?

          # If he is still in events (if he is not dead)
          if events?

            ia_play_events( game_board, prof )

            # Break out of the investigators loop
            if game_board.prof_asked_for_fake_cities?
              # If investigator is still alive
              events_done! if events?
              return :break
            else
              # When the turn of the investigator is finished we need to check for a prof fight
              check_for_prof_to_fight_in_city( game_board, prof )
              events_done! if events?
            end
          end
        end
      end
    end
  end
end
