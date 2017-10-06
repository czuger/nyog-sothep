module GameCore
  module Ia
    # Included in Investigator
    module InvestigatorMovement

      include GameCore::Assertions

      # imt is the InvestigatorMovementTarget class
      def ia_play_movements( game_board, imt )
        if got_to_psy_check( game_board )
          go_to_psy( game_board )
        else
          ia_invest_random_move( game_board, imt )
          movement_done!
        end
      end

      private

      def ia_invest_random_move( game_board, imt )

        if game_board.nyog_sothep_invoked

          # We check a city for investigators to met
          unless game_board.nyog_sothep_repelling_city_code_name

            final_city_code_name = GameCore::Ia::InvestigatorChooseBestCityToMeet.new( game_board ).get_best_city
            if final_city_code_name
              game_board.nyog_sothep_repelling_city_code_name = final_city_code_name
            else
              raise 'No city avaliable to summon Nyog Sothep'
            end
          end

          if self.ia_target_destination_code_name != game_board.nyog_sothep_repelling_city_code_name
            self.ia_target_destination_code_name = game_board.nyog_sothep_repelling_city_code_name
          end

        else
          # If does not have a destination or we are at destination or our destination has been destroyed, then we chose one
          # Also if investigator has a weapon he recompute it's target in order to chase the prof
          # (as the prof move each turn he need to recompute it's target each move)
          if weapon || self.ia_target_destination_code_name.nil? ||
             self.ia_target_destination_code_name == current_location_code_name ||
            imt.exclusion_city_codes_names_list.include?( self.ia_target_destination_code_name )

            self.ia_target_destination_code_name = imt.select_new_target( self )
          end
          # Otherwise, we continue the same way
        end

        if !game_board.nyog_sothep_invoked && !weapon && self.ia_target_destination_code_name == current_location_code_name
          raise "Current position should not be the same as target : #{self.ia_target_destination_code_name} == #{current_location_code_name}"
        end

        move_to_target( game_board, imt )
      end

      def move_to_target( game_board, imt )

        puts "#{translated_name} is at #{current_location_code_name} and is heading to #{self.ia_target_destination_code_name}" if IInvestigator::DEBUG_MOVEMENTS
        next_step_code_name, _ = GameCore::Ia::BfsAlgo.find_next_dest_to_goal(
          current_location_code_name, self.ia_target_destination_code_name,
          imt.exclusion_city_codes_names_list + [ forbidden_city_code_name ] )

        if next_step_code_name
          raise "Next movement is forbidden. Forbidden_city = #{forbidden_city_code_name.inspect}, next_step_code_name = #{next_step_code_name.inspect}" if next_step_code_name == forbidden_city_code_name
          raise "Next movement is destroyed city. next_step_code_name = #{next_step_code_name.inspect}" if imt.exclusion_city_codes_names_list.include?( next_step_code_name )

          if next_step_code_name
            # When nyog sothep is invoked, investigators have to meet, so we don't move once we are at the good destination
            next_step_code_name = GameCore::Map::Location.get_location( next_step_code_name )
            regular_move_token( game_board, self, next_step_code_name  )
          end
        else
          # TODO : translate event
          LLog.log( game_board, self, :cant_move )
        end

      end

      def regular_move_token( gb, token, dest_loc )

        assert_regular_movement_allowed( token.current_location, dest_loc )

        if cross_border_allowed( gb, token, dest_loc )

          token.last_location_code_name = token.current_location_code_name
          token.current_location_code_name = dest_loc.code_name

          LLog.log_investigator_movement( gb, token, dest_loc.code_name )

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
              event = 'border_control'
              LLog.log( gb, self, event )
              border_allowed = false
            end
          end
        end

        border_allowed
      end

      def got_to_psy_check( game_board )

        # If nyog sothep has been invoked
        if game_board.nyog_sothep_invoked
          # If we are at the place to repel it, then we go to the psy
          if current_location_code_name == game_board.nyog_sothep_repelling_city_code_name
            return true
          end
        # If san is below 5 and investigator is in city, then he/she goes to the psy
        elsif san < 5 && current_location.city?
          return true
        end

        false

      end
    end
  end
end