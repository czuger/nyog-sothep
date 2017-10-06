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

        log_encounter( investigator,'habitants' )
        investigator.update( medaillon: false, forbidden_city_code_name: investigator.current_location_code_name )
        investigator.goes_back( self )
        replace_encounter_in_monsters_stack( encounter )
      end
    end

    def resolve_encounter_choses_brume( investigator, encounter )
      log_encounter( investigator,'choses_brume' )
      investigator.entering_misty_things!
      replace_encounter_in_monsters_stack( encounter )
    end

    def resolve_encounter_fanatiques( investigator, encounter )
      unless investigator.weapon
        log_encounter(  investigator,'fanatiques.no_weapon' )
        investigator.goes_back( self )
        monster_spotted( encounter )
      else
        roll = GameCore::Dices.d6
        roll += 1 if investigator.medaillon
        if roll <= 5
          log_encounter(  investigator,'fanatiques.weapon_fail' )
          investigator.goes_back( self )
          monster_spotted( encounter )
        elsif roll == 6
          investigator.loose_san( self, 2 )
          investigator.update( medaillon: true )
          log_encounter(  investigator,'fanatiques.weapon_success', 2, investigator.san )
          encounter_destroyed( encounter )
        else
          log_encounter(  investigator,'fanatiques.weapon_critical' )
          investigator.update( medaillon: true )
          encounter_destroyed( encounter )
        end
      end
    end

    def resolve_encounter_profonds( investigator, encounter )
      if investigator.sign
        san_loss = 1
        investigator.update_attribute( :sign, false )
        investigator.loose_san( self, san_loss )
        replace_encounter_in_monsters_stack( encounter )
        log_encounter(  investigator,'profonds.sign', san_loss, investigator.san )
      else
        san_loss = 3
        investigator.loose_san( self, san_loss )
        monster_spotted( encounter )
        log_encounter(  investigator,'profonds.no_sign', san_loss, investigator.san )
      end
    end

    def resolve_encounter_goules( investigator, encounter )
      if investigator.weapon
        san_loss = 3
        san_loss = 2 if GameCore::Dices.d6 >= 5
        san_loss -= 1 if investigator.sign
        investigator.loose_san( self, san_loss )
        replace_encounter_in_monsters_stack( encounter )
        log_encounter(  investigator,'goules.weapon', san_loss, investigator.san )
      else
        san_loss = 4
        investigator.loose_san( self, san_loss )
        monster_spotted( encounter )
        log_encounter(  investigator,'goules.no_weapon', san_loss, investigator.san )
      end
    end

    def resolve_encounter_reves( investigator, encounter )
      san_loss = 2
      investigator.loose_san( self, san_loss )
      replace_encounter_in_monsters_stack( encounter )
      log_encounter(  investigator,'reves', san_loss, investigator.san )
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

    def log_encounter( investigator, code, san_loss = nil, cur_san = nil )
      data = {}
      data[:san_loss] = san_loss if san_loss
      data[:cur_san] = cur_san if cur_san

      code = 'encounter.' + code
      name_translation_method = I18n.t( code + '.name_translation' )

      LLog.log( self, investigator,code, true, data,
                true, false, name_translation_method )
    end

  end
end
