module GameCore
  module Map
    class City < GameCore::Map::Location

      def self.all
        load_locations
        @@locations.keys.map{ |k| Location.get_location( k ) }.reject{ |e| e.water_area? }.compact
      end

      def self.all_codes_names
        load_locations
        @@locations.keys
      end

      # exclusion_list is an array of string
      def self.random_city_code_name( exclusion_list )
        ( all_codes_names - exclusion_list.map( &:to_sym ) ).sample
      end

      def city?
        true
      end

      def outgoing_roads
      end

    end
  end
end
