module GameCore

  # Included in Investigator
  module InvestigatorFight

    def check_for_prof_to_fight_in_city( game_board, prof )

      if prof.current_location == current_location
        prof.spotted( game_board )

        if weapon
          # If we have a weapon : we fight (even if we have the medaillon)
          prof.fight( game_board, self )
        elsif medaillon
          # If we have no weapon, we hide (with the medaillon)
          # nothing
        else
          # No choice : must fight the prof without weapon
          prof.fight( game_board, self )
        end
      end
    end

  end
end
