module GameCore
  module Ia
    # Included in Investigator
    module InvestigatorMovement

      include GameCore::Movement

      private

      def ia_invest_random_move( game_board )

        # If we does not have a destination or we are at destination, then we chose one
        if !ia_target_destination || ia_target_destination.code_name == current_location.code_name

          # We chase the professor only if we have a weapon. Otherwise, we walk randomly
          if weapon && game_board.i_inv_target_positions.count > 0
            position = game_board.i_inv_target_positions.reject{ |e| e.position.code_name == current_location.code_name}.sample&.position
          end

          unless position
            position = CCity.all.reject{ |e| e.code_name == current_location.code_name}.sample # Random first
          end

          self.ia_target_destination = position
        end

        # p ia_target_destination

        next_step = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( current_location, ia_target_destination, forbidden_city: forbidden_city )

        raise "Next movement is forbidden. Forbidden_city = #{forbidden_city.code_name}, next_step = #{next_step.code_name}" if next_step.code_name == forbidden_city&.code_name

        regular_move_token( game_board, self, next_step  )
      end

    end

    private
  end
end