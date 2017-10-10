module GameCore

  # Included in Investigator
  module IInvestigatorFight

    def check_for_prof_to_fight_in_city( game_board, prof, prof_position_finder )

      # We fight only in cities.
      if GameCore::Map::Location.get_location( current_location_code_name ).city?
        if current_location_code_name == prof.current_location_code_name

          prof.spotted( game_board, self, prof_position_finder )

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
end
