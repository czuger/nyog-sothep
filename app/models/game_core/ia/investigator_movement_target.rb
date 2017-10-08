module GameCore
  module Ia

    # This class is used to evaluate the prof position and select a target for the investigator
    # accordingly to he chasing the prof or not.
    class InvestigatorMovementTarget

      attr_reader :exclusion_city_codes_names_list

      def initialize( game_board, prof_position_finder, exclusion_city_codes_names_list = [] )
        @game_board = game_board
        @exclusion_city_codes_names_list = exclusion_city_codes_names_list + game_board.g_destroyed_cities.pluck( :city_code_name )
        @prof_position_finder = prof_position_finder

        @prof_positions_covered_by_inv = Set.new

      end

      # exclusion_city_code_name : a list of all unavaliable city (destroyed, forbiden, etc ...). An array of strings
      def select_new_target( investigator )

        refresh_prof_positions

        if investigator.weapon
          # If we have a weapon, then we chase the prof
          target_position_code_name = guess_chasing_prof_location
          puts "#{translated_name} now change target to chase the professor at #{target_position_code_name}" if IInvestigator::DEBUG_MOVEMENTS

          # If we couldn't get the prof position, we pick one randomly
          target_position_code_name = choose_random_location( investigator ) unless target_position_code_name
        else
          # Otherwise we flee from him
          prof_position_code_name = guess_avoiding_prof_location

          # If we couldn't get the prof position, we pick one randomly
          prof_position_code_name = choose_random_location( investigator ) unless prof_position_code_name

          # TODO : We need to find a city far away from the prof
          far_cities_dict = GameCore::Ia::BfsAlgo.find_cities_around_city( prof_position_code_name, 5, @exclusion_city_codes_names_list )
          far_cities_codes_names_array = far_cities_dict[5]
          target_position_code_name = far_cities_codes_names_array.sample
        end

        target_position_code_name
      end

      private

      def choose_random_location( investigator )
        exclusion_list = @exclusion_city_codes_names_list +
          [ investigator.current_location_code_name, investigator.last_location_code_name, investigator.ia_target_destination_code_name ]
        target_position_code_name = GameCore::Map::City.random_city_code_name( exclusion_list )

        raise "Can't find a random position : exclusion_list = #{exclusion_list.inspect}" unless target_position_code_name
        puts "#{translated_name} now change target randomly at #{target_position_code_name}" if IInvestigator::DEBUG_MOVEMENTS

        target_position_code_name
      end

      def guess_avoiding_prof_location
        return @first_prof_position if @first_prof_position

        unless @most_probable_prof_locations.empty?
          return @most_probable_prof_locations.sample
        end
      end

      def guess_chasing_prof_location
        return @first_prof_position if @first_prof_position

        # So here we don't have a sure prof position, we will guess it
        # In fact we send investigators everywhere we have information
        @most_probable_prof_locations.each do |pp|
          # If nobody is already chasing the prof on that pos, then we go there
          unless @prof_positions_covered_by_inv.include?( pp )
            @prof_positions_covered_by_inv << pp
            return pp
          end
        end

        # If all slots are filled, then choose randomly
        unless @most_probable_prof_locations.empty?
          return @most_probable_prof_locations.sample
        end
      end

      def refresh_prof_positions
        @most_probable_prof_locations = @prof_position_finder.get_prof_positions( @game_board.turn )
        @most_probable_prof_locations = [] unless @most_probable_prof_locations
        @first_prof_position = @most_probable_prof_locations.first if @most_probable_prof_locations&.count == 1
      end

    end
  end
end
