module GameCore
  module Ia
    # Included in Investigator
    module InvestigatorMovement

      include GameCore::Assertions

      def ia_play_movements( game_board, _ )
        if got_to_psy_check( game_board )
          go_to_psy(game_board )
        else
          ia_invest_random_move( game_board )
          movement_done!
        end
      end

      private

      def ia_invest_random_move( game_board )

        destroyed_cities_codes_names = game_board.g_destroyed_cities.pluck( :city_code_name )
        destroyed_cities_codes_names.map! &:to_sym

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
          # If we does not have a destination or we are at destination or our destination has been destroyed, then we chose one
          if self.ia_target_destination_code_name.nil? ||
             self.ia_target_destination_code_name == current_location_code_name ||
             destroyed_cities_codes_names.include?( self.ia_target_destination_code_name.to_sym )

            self.ia_target_destination_code_name = select_new_target( game_board, destroyed_cities_codes_names )
          end
          # Otherwise, we continue the same way
        end

        if !game_board.nyog_sothep_invoked && self.ia_target_destination_code_name == current_location_code_name
          raise "Current position should not be the same as target : #{current_position_code_name} == #{goal_code_name}"
        end

        move_to_target( game_board, destroyed_cities_codes_names )
      end

      def move_to_target( game_board, destroyed_cities_codes_names )

        puts "#{translated_name} is at #{current_location_code_name} and is heading to #{self.ia_target_destination_code_name}"
        next_step_code_name, _ = GameCore::Ia::BfsAlgo.find_next_dest_to_goal(
          current_location_code_name, self.ia_target_destination_code_name,
          forbidden_city_code_name: forbidden_city_code_name, destroyed_cities_codes_names: destroyed_cities_codes_names )

        if next_step_code_name
          raise "Next movement is forbidden. Forbidden_city = #{forbidden_city_code_name.inspect}, next_step_code_name = #{next_step_code_name.inspect}" if next_step_code_name == forbidden_city_code_name
          raise "Next movement is destroyed city. next_step_code_name = #{next_step_code_name.inspect}" if destroyed_cities_codes_names.include?( next_step_code_name )

          if next_step_code_name
            # When nyog sothep is invoked, investigators have to meet, so we don't move once we are at the good destination
            next_step_code_name = GameCore::Map::Location.get_location( next_step_code_name )
            regular_move_token( game_board, self, next_step_code_name  )
          end
        else
          # TODO : translate event
          event_log = EEventLog.log( game_board, self, "#{translated_name} ne trouve pas de chemin de #{current_location_code_name} vers #{self.ia_target_destination_code_name}" )
          EEventLogSummary.log( game_board, self, :inv_cant_move, { investigator_name: translated_name }, event_log )
        end

      end

      def select_new_target( game_board, destroyed_cities_codes_names )
        # We chase the professor only if we have a weapon. Otherwise, we walk randomly
        if weapon && game_board.i_inv_target_positions.count > 0
          target_position_code_name = game_board.i_inv_target_positions.reject{ |e| e.position_code_name == current_location_code_name}.sample&.position_code_name
          puts "#{translated_name} now change target to chase the professor at #{target_position_code_name}"
        end

        # If Ia didn't find an interesant target, then we choose one randomly
        unless target_position_code_name
          exclusion_list = destroyed_cities_codes_names + [ current_location_code_name.to_sym ]
          target_position_code_name = GameCore::Map::City.random_city_code_name( exclusion_list )
          raise "Can't find a random position : exclusion_list = #{exclusion_list.inspect}" unless target_position_code_name
          puts "#{translated_name} now change target randomly at #{target_position_code_name}"
        end

        target_position_code_name
      end

      def regular_move_token( gb, token, dest_loc )

        assert_regular_movement_allowed( token.current_location, dest_loc )

        if cross_border_allowed( gb, token, dest_loc )

          token.last_location_code_name = token.current_location_code_name
          token.current_location_code_name = dest_loc.code_name

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