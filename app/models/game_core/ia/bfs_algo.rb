module GameCore
  module Ia

    class BfsAlgo

      def self.find_next_dest_to_goal( current_position_code_name, goal_code_name, forbidden_city_code_name: nil, destroyed_cities_codes_names: [] )

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
            next if next_location == forbidden_city_code_name
            next if destroyed_cities_codes_names.include?( next_location )
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

    end
  end
end
