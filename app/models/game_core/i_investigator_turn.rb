module GameCore
  # Included in IInvestigator
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
      if san <= 0
        die( game_board )
        return false
      end
      true
    end

    def gain_san( game_board, san_amount )
      increment!( :san, san_amount )
    end

    def goes_back( game_board )
      ll = last_location_code_name
      self.current_location_code_name = ll
      # save!
      LLog.log_investigator_movement( game_board, self, ll, direction: :return )
    end

    private

    def gain_san_from_great_psy()
      increment!( :san, 5 )
    end

    def loose_san_from_misty_things( game_board )
      increment!( :san, -1 )
      LLog.log( game_board, self,'encounter.back_from_misty_things', true ) if game_board
    end

    def die( game_board )
      LLog.log( game_board, self,'actions.result.crazy', true, {},
                true, true )
      game_board.p_monster_positions.create!(
        location_code_name: current_location_code_name, code_name: 'fanatiques', discovered: true, token_rotation: rand( -15 .. 15 ) )
      aasm_die!
    end

  end
end
