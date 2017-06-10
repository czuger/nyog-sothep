module GameCore
  module IInvestigatorTurn

    def next_turn

      if going_to_great_psy?
        finishing_turn_at_a_great_psy!

      elsif at_a_great_psy?
        back_from_great_psy!

      elsif at_a_psy?
        back_from_psy!

      elsif to_misty_things?
        finishing_turn_in_misty_things!

      elsif at_misty_things?
        back_from_misty_things!

      else
        new_turn!

      end
    end

    def loose_san( game_board, san_amount )
      decrement!( :san, san_amount )
      # EEventLog.log( game_board, self,I18n.t( 'actions.result.perte_san', san: san_amount,  investigator_name: translated_name, final_san: san ) )
      if san <= 0
        die( game_board )
        return false
      end
      true
    end

    def gain_san( game_board, san_amount )
      loose_san( game_board, -san_amount )
    end

    def goes_back( game_board )
      ll = last_location
      self.current_location = ll
      # save!
      EEventLog.log_investigator_movement( game_board, self, ll, direction: :return )
    end

    private

    def gain_san_from_great_psy()
      increment!( :san, 5 )
    end

    def loose_san_from_misty_things( game_board )
      increment!( :san, -1 )
      EEventLog.log( game_board, self,I18n.t( "encounter.back_from_misty_things.#{gender}" ) ) if game_board
    end

    def die( game_board )
      EEventLog.log( game_board, self,I18n.t( "actions.result.crazy.#{gender}", investigator_name: translated_name ) )
      game_board.p_monster_positions.create!(
        location_code_name: current_location_code_name, code_name: 'fanatiques', discovered: true, token_rotation: rand( -15 .. 15 ) )
      aasm_die!
    end

  end
end
