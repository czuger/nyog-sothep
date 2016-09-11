module GameLogic
  module Encounters

    def check_encounter
      encounter = PMonsterPosition.where( location_id: @current_investigator.current_location_id ).first
      if encounter
        encounter.update_attribute( :discovered, true )
      end
    end

  end
end