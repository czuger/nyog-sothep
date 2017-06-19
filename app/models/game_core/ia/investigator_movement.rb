module GameCore
  module Ia
    # Included in Investigator
    module InvestigatorMovement

      include GameCore::Assertions

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
            position_code_name = GameCore::Map::City.random_city_code_name( [ current_location_code_name ] )
          end

          self.ia_target_destination_code_name = position_code_name
        end

        # p ia_target_destination

        next_step_code_name, _ = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( current_location_code_name, ia_target_destination_code_name, forbidden_city_code_name )

        raise "Next movement is forbidden. Forbidden_city = #{forbidden_city_code_name.inspect}, next_step_code_name = #{next_step_code_name.inspect}" if next_step_code_name == forbidden_city_code_name

        if next_step_code_name
          next_step_code_name = GameCore::Map::Location.get_location( next_step_code_name )
          regular_move_token( game_board, self, next_step_code_name  )
        end
      end

      def regular_move_token( gb, token, dest_loc )

        assert_regular_movement_allowed( token.current_location, dest_loc )

        if cross_border_allowed( gb, token, dest_loc )

          token.last_location_code_name = token.current_location_code_name

          token.current_location_code_name = dest_loc.code_name
          token.save!

          EEventLog.log_investigator_movement( gb, token, dest_loc.code_name )

          return true
        end

        false
      end

      def cross_border_allowed( gb, token, dest_loc )

        border_allowed = true

        if dest_loc.city? && token.current_location.city?
          if GameCore::Map::BordersCrossings.check?( token.current_location_code_name, dest_loc.code_name )
            dice = GameCore::Dices.d6
            if dice >= 5
              inv_name = I18n.t( "investigators.#{token.code_name}" )
              event = I18n.t( "border_control.#{token.gender}", investigator_name: inv_name )
              EEventLog.log( gb, self, event )
              border_allowed = false
            end
          end
        end

        border_allowed
      end

    end
  end
end