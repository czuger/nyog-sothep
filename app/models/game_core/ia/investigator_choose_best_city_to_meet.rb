module GameCore
  module Ia

    class InvestigatorChooseBestCityToMeet

      def initialize( game_board )
        @candidates_cities_codes_names_weights = {}
        @max_weight = 0
        @game_board = game_board
      end

      def get_best_city
        @candidates_cities_codes_names_weights.clear
        @max_weight = 0

        check_paths_for_cities
        select_final_city
      end

      private

      def select_final_city
        @candidates_cities_codes_names_weights.delete_if {|_, value| value != @max_weight }
        @candidates_cities_codes_names_weights.keys.sample
      end

      def check_paths_for_cities
        candidates_cities_codes_names = GameCore::Map::City.all_codes_names
        destroyed_cities_codes_names = @game_board.g_destroyed_cities.pluck( :city_code_name ).map( &:to_sym )
        candidates_cities_codes_names.reject{ |cn| destroyed_cities_codes_names.include?( cn ) }

        @game_board.alive_investigators.each do |investigator|
          candidates_cities_codes_names.each do |cn|

            next if investigator.current_location_code_name.to_sym == cn

            next_step, _ = GameCore::Ia::BfsAlgo.find_next_dest_to_goal(
              investigator.current_location_code_name,
              cn, destroyed_cities_codes_names: destroyed_cities_codes_names )

            if next_step
              # We found a path to the city
              @candidates_cities_codes_names_weights[ cn ] ||= 0
              @candidates_cities_codes_names_weights[ cn ] += 1
              @max_weight = [ @max_weight, @candidates_cities_codes_names_weights[ cn ] ].max
            end
          end
        end
      end

    end
  end
end

