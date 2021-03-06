module GameCore
  module Ia

    class BfsAlgo

      def self.find_next_dest_to_goal( current_position_code_name, goal_code_name, exclusion_cities_codes_names = [] )

        raise 'Goal is nil' unless goal_code_name

        current_position_code_name = current_position_code_name.to_sym
        goal_code_name = goal_code_name.to_sym

        frontier = []
        frontier << current_position_code_name
        came_from = {}
        came_from[ current_position_code_name ] = nil

        until frontier.empty?
          current_name = frontier.shift

          if current_name == goal_code_name
            break
          end

          GameCore::Map::Location.destinations_codes_names_from_code_name( current_name ).each do |next_location|
            next if exclusion_cities_codes_names.include?( next_location.to_s )
            unless came_from.has_key?( next_location )
              frontier << next_location
              came_from[ next_location ] = current_name
            end
          end
        end

        back_token = goal_code_name
        next_step = nil
        distance = 0

        # pp came_from

        while back_token != current_position_code_name
          next_step = back_token
          # p next_step
          back_token = came_from[ back_token ]
          distance += 1
          break if next_step == nil
        end

        [ next_step, distance ]
      end

      def self.find_cities_around_city( current_position_code_name, distance, exclusion_cities_codes_names = [] )

        current_position_code_name = current_position_code_name.to_sym

        cities_to_check_cn = [ current_position_code_name ]
        processed_locations = Set.new( [ current_position_code_name ] )
        near_cities_cn = {}

        step = 1
        while step <= distance

          near_cities_cn[ step ] ||= []
          cities_to_check_cn_this_step = cities_to_check_cn.clone
          cities_to_check_cn = []

          for city_cn in cities_to_check_cn_this_step
            neighbour_locations_cn = GameCore::Map::Location.get_location( city_cn ).destinations
            neighbour_locations_cn.reject!{ |e| processed_locations.include?( e.code_name ) || exclusion_cities_codes_names.include?( e.to_s ) }

            neighbour_locations_cn.each do |nc|
              cn = nc.code_name
              unless nc.water_area?
                near_cities_cn[ step ] << cn
              end
              
              processed_locations << cn
              cities_to_check_cn << cn
            end
          end

          step += 1
        end
        near_cities_cn
      end

    end
  end
end
