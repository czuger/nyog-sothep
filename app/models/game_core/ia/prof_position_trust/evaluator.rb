module GameCore
  module Ia
    module ProfPositionTrust

      # This class is used to reevaluate the trust value in the i_inv_target_position
      class Evaluator

        def initialize( game_board )
          @game_board = game_board
          @target_positions = @game_board.i_inv_target_positions.order( 'turn DESC' ).pluck( :turn, :trust, :position_code_name )
        end

        def refresh_all
          eliminate_improbable_inputs_in_turn
          eliminate_improbable_movements
        end

        private

        def eliminate_improbable_inputs_in_turn
          current_target_positions = @target_positions.select{ |e| e[0] == @game_board.turn }

          duplicates_current_positions = {}
          multi_positions_in_turn = []
          all_positions_in_turn = []

          # We count the number of positions in a turn (in case we have more than one fake position during one turn)
          current_target_positions.each do |ctp|
            _, _, position_code_name = ctp
            duplicates_current_positions[ position_code_name ] ||= 0
            duplicates_current_positions[ position_code_name ]+= 1

            multi_positions_in_turn << position_code_name if duplicates_current_positions[ position_code_name ] > 1
            all_positions_in_turn << position_code_name
          end

          multi_positions_in_turn.uniq!
          all_positions_in_turn.uniq!

          if multi_positions_in_turn.size == 1
            # Hooray the player is stoopid. We know the prof position.
            prof_pos = multi_positions_in_turn
            @game_board.i_inv_target_positions.where( turn: @game_board.turn ).update_all( trust: 0 )
            @game_board.i_inv_target_positions.where( turn: @game_board.turn, position_code_name: prof_pos ).update_all( trust: 1 )
          elsif
            multi_positions_in_turn.size > 1
            bad_positions = multi_positions_in_turn - all_positions_in_turn
            # We check for positions that have changed during turn
            unless bad_positions.empty?
              # TODO : ajouter un index sur turn
              # If we found some, then they are bad positions.
              @game_board.i_inv_target_positions.where( turn: @game_board.turn, position_code_name: bad_positions ).update_all( trust: 0 )
            end
          end

        end

        def eliminate_improbable_movements

        end

      end
    end
  end
end
