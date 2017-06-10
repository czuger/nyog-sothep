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
        EEventLog.log( self, investigator,I18n.t( 'encounter.habitants', investigator_name: investigator.translated_name ) )
        investigator.update( medaillon: false, forbidden_city_code_name: investigator.current_location_code_name )
        investigator.goes_back( self )
        replace_encounter_in_monsters_stack( encounter )
      end
    end

    def resolve_encounter_choses_brume( investigator, encounter )
      EEventLog.log( self, investigator,I18n.t( 'encounter.choses_brume', investigator_name: investigator.translated_name ) )
      investigator.entering_misty_things!
      replace_encounter_in_monsters_stack( encounter )
    end

    def resolve_encounter_fanatiques( investigator, encounter )
      log = I18n.t( 'encounter.fanatiques.common', investigator_name: investigator.translated_name )
      unless investigator.weapon
        log << I18n.t( 'encounter.fanatiques.no_weapon' )
        EEventLog.log( self, investigator,log )
        investigator.goes_back( self )
        monster_spotted( encounter )
      else
        roll = GameCore::Dices.d6
        roll += 1 if investigator.medaillon
        if roll <= 5
          log << I18n.t( 'encounter.fanatiques.weapon_fail' )
          EEventLog.log( self, investigator,log )
          investigator.goes_back( self )
          monster_spotted( encounter )
        elsif roll == 6
          log << I18n.t( 'encounter.fanatiques.weapon_success' )
          EEventLog.log( self, investigator,log )
          investigator.loose_san( self, 2 )
          investigator.update( medaillon: true )
          encounter_destroyed( encounter )
        else
          log << I18n.t( 'encounter.fanatiques.weapon_critical' )
          EEventLog.log( self, investigator,log )
          investigator.update( medaillon: true )
          encounter_destroyed( encounter )
        end
      end
    end

    def resolve_encounter_profonds( investigator, encounter )
      log = I18n.t( 'encounter.profonds.common', investigator_name: investigator.translated_name )
      if investigator.sign
        san_loss = 1
        replace_encounter_in_monsters_stack( encounter )
        investigator.update_attribute( :sign, false )
        log << I18n.t( 'encounter.profonds.sign' )
      else
        san_loss = 3
        monster_spotted( encounter )
        log << I18n.t( 'encounter.profonds.no_sign' )
      end
      investigator.loose_san( self, san_loss )
      EEventLog.log( self, investigator,log )
    end

    def resolve_encounter_goules( investigator, encounter )
      log = I18n.t( 'encounter.goules.common', investigator_name: investigator.translated_name )
      if investigator.weapon
        san_loss = 3
        san_loss = 2 if GameCore::Dices.d6 >= 5
        san_loss -= 1 if investigator.sign
        replace_encounter_in_monsters_stack( encounter )
        log << I18n.t( 'encounter.goules.weapon', san: san_loss )
      else
        san_loss = 4
        monster_spotted( encounter )
        log << I18n.t( 'encounter.goules.no_weapon' )
      end
      investigator.loose_san( self, san_loss )
      EEventLog.log( self, investigator,log )
    end

    def resolve_encounter_reves( investigator, encounter )
      EEventLog.log( self, investigator,I18n.t( 'encounter.reves', investigator_name: investigator.translated_name ) )
      investigator.loose_san( self, 2 )
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
