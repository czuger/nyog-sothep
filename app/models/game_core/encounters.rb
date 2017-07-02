module GameCore
  # Included in GGameBoard
  module Encounters

    def resolve_encounter( investigator )
      encounter = p_monster_positions.find_by_location_code_name( investigator.current_location_code_name )
      if encounter
        send( "resolve_encounter_#{encounter.code_name}", investigator, encounter )
      end
    end

    private

    def resolve_encounter_habitants( investigator, encounter )
      if investigator.medaillon
        LLog.log( self, investigator,'encounter.habitants', {} )
        investigator.update( medaillon: false, forbidden_city_code_name: investigator.current_location_code_name )
        investigator.goes_back( self )
        replace_encounter_in_monsters_stack( encounter )
      end
    end

    def resolve_encounter_choses_brume( investigator, encounter )
      LLog.log( self, investigator,'encounter.choses_brume', {} )
      investigator.entering_misty_things!
      replace_encounter_in_monsters_stack( encounter )
    end

    def resolve_encounter_fanatiques( investigator, encounter )
      log = 'encounter.fanatiques.common'
      unless investigator.weapon
        log << 'encounter.fanatiques.no_weapon'
        LLog.log( self, investigator,log, {} )
        investigator.goes_back( self )
        monster_spotted( encounter )
      else
        roll = GameCore::Dices.d6
        roll += 1 if investigator.medaillon
        if roll <= 5
          log << 'encounter.fanatiques.weapon_fail'
          LLog.log( self, investigator,log, {} )
          investigator.goes_back( self )
          monster_spotted( encounter )
        elsif roll == 6
          log << 'encounter.fanatiques.weapon_success'
          LLog.log( self, investigator,log, {} )
          investigator.loose_san( self, 2 )
          investigator.update( medaillon: true )
          encounter_destroyed( encounter )
        else
          log << 'encounter.fanatiques.weapon_critical'
          LLog.log( self, investigator,log, {} )
          investigator.update( medaillon: true )
          encounter_destroyed( encounter )
        end
      end
    end

    def resolve_encounter_profonds( investigator, encounter )
      log = 'encounter.profonds.common'
      if investigator.sign
        san_loss = 1
        replace_encounter_in_monsters_stack( encounter )
        investigator.update_attribute( :sign, false )
        log << 'encounter.profonds.sign'
      else
        san_loss = 3
        monster_spotted( encounter )
        log << 'encounter.profonds.no_sign'
      end
      # TODO : créer les logs pour chaque partie de if. Faire la distinction entre ce qui soit avoir un résumé ou non.
      LLog.log( self, investigator,log, {} )
      investigator.loose_san( self, san_loss )
    end

    def resolve_encounter_goules( investigator, encounter )
      log = 'encounter.goules.common'
      if investigator.weapon
        san_loss = 3
        san_loss = 2 if GameCore::Dices.d6 >= 5
        san_loss -= 1 if investigator.sign
        replace_encounter_in_monsters_stack( encounter )
        log << 'encounter.goules.weapon'
      else
        san_loss = 4
        monster_spotted( encounter )
        log << 'encounter.goules.no_weapon'
      end
      LLog.log( self, investigator,log, {} )
      investigator.loose_san( self, san_loss )

    end

    def resolve_encounter_reves( investigator, encounter )
      event_log = LLog.log( self, investigator,'encounter.reves', {} )
      investigator.loose_san( self, 2, event_log )
      replace_encounter_in_monsters_stack( encounter )
    end

    # Common methods

    def monster_spotted( encounter )
      encounter.update( discovered: true )
    end

    def replace_encounter_in_monsters_stack( encounter )
      m_monsters.create!( code_name: encounter.code_name )
      encounter.destroy!
    end

    def encounter_destroyed( encounter )
      encounter.destroy!
    end


  end
end
