module GameCore
  module Ia
    module InvestigatorMovement

      include GameCore::Movement

      private

      def ia_invest_random_move( game_board )

        # If we does not have a destination or we are at destination, then we chose one
        # p ia_target_destination
        if !ia_target_destination || ia_target_destination.code_name == current_location.code_name

          if game_board.i_inv_target_positions.count > 0
            self.ia_target_destination = game_board.i_inv_target_positions.sample.position
          else
            self.ia_target_destination = CCity.all.reject{ |e| e.code_name == current_location.code_name}.sample # Random first
          end
        end

        # p ia_target_destination

        next_step = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( current_location, ia_target_destination )

        regular_move_token( game_board, self, next_step  )
      end

    end
  end
end