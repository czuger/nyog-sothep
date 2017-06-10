module GameCore
  module Map
    class City < GameCore::Map::Location

      def self.all
        self.load_data
        @@locations.keys.map{ |k| Location.get_location( k ) }
      end

      def self.all_codes_names
        self.load_data
        @@locations.keys
      end

      def self.random_city_code_name
        all_codes_names.sample
      end

      def city?
        true
      end

      def outgoing_roads
      end

    end
  end
end
