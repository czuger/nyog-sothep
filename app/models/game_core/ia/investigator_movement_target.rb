module GameCore
  module Ia

    # This class is used to evaluate the prof position and select a target for the investigator
    # accordingly to he chasing the prof or not.
    class InvestigatorMovementTarget

      attr_reader :exclusion_city_codes_names_list

      def initialize( game_board, exclusion_city_codes_names_list = [] )
        @game_board = game_board
        @exclusion_city_codes_names_list = exclusion_city_codes_names_list + game_board.g_destroyed_cities.pluck( :city_code_name )

        @prof_positions_covered_by_inv = Set.new

        @trust_evaluator = GameCore::Ia::ProfPositionTrust::Evaluator.new( @game_board )
        refresh_prof_positions
      end

      # exclusion_city_code_name : a list of all unavaliable city (destroyed, forbiden, etc ...). An array of strings
      def select_new_target( investigator )

        @trust_evaluator.refresh_all

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
          prof_position_code_name = choose_random_location( investigator ) unless target_position_code_name

          # TODO : We need to find a city far away from the prof
          far_cities_dict = GameCore::Ia::BfsAlgo.find_cities_around_city( investigator.current_location_code_name, 5, @exclusion_city_codes_names_list )
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
        @prof_positions.each do |pp|
          # If nobody is already chasing the prof on that pos, then we go there
          unless @prof_positions_covered_by_inv.include?( pp[0] )
            @prof_positions_covered_by_inv << pp[0]
            return pp[0]
          end
        end

        # If all slots are filled, then choose randomly
        unless @prof_positions.empty?
          return @prof_positions.sample[0]
        end
      end

      def refresh_prof_positions
        # @prof_positions is an array of [ [ 'oxford', 0.9 ], 'lowel', 0.8 ], etc ... ]
        @prof_positions = @game_board.i_inv_target_positions.order( 'trust DESC' ).pluck( :position_code_name, :trust )
        @prof_positions.reject!{ |e| @exclusion_city_codes_names_list.include?( e[0] ) }

        first_prof_position = @prof_positions.first
        # If we know where the prof is, every chasing investigator go there
        @first_prof_position = first_prof_position[0] if first_prof_position && first_prof_position[1] >= 0.8

        highest_trust = @prof_positions.map{ |e| e[1] }.max
        @most_probable_prof_locations = []
        @prof_positions.each do |e|
          @most_probable_prof_locations << e[0] if e[1] >= highest_trust - 0.1
        end

      end

    end
  end
end
