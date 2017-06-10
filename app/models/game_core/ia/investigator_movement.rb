module GameCore
  module Ia
    # Included in Investigator
    module InvestigatorMovement

      include GameCore::Movement

      def ia_play_movements( game_board, _ )
        if san < 5 && current_location.city?
          # If san is below 5 and investigator is in city, then he/she goes to the psy
          go_to_psy(game_board )
        else
          ia_invest_random_move( game_board )
          movement_done!
        end
      end

      private

      def ia_invest_random_move( game_board )

        # If we does not have a destination or we are at destination, then we chose one
        if !ia_target_destination_code_name || ia_target_destination_code_name == current_location_code_name

          # We chase the professor only if we have a weapon. Otherwise, we walk randomly
          if weapon && game_board.i_inv_target_positions.count > 0
            position_code_name = game_board.i_inv_target_positions.reject{ |e| e.position_code_name == current_location_code_name}.sample&.position_code_name
          end

          unless position_code_name
            position_code_name = GameCore::Map::City.random_city_code_name
          end

          self.ia_target_destination_code_name = position_code_name
        end

        # p ia_target_destination

        next_step_code_name = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( current_location_code_name, ia_target_destination_code_name, forbidden_city_code_name )

        raise "Next movement is forbidden. Forbidden_city = #{forbidden_city_code_name.inspect}, next_step_code_name = #{next_step_code_name.inspect}" if next_step_code_name == forbidden_city_code_name

        regular_move_token( game_board, self, next_step_code_name  )
      end

    end



  end
end