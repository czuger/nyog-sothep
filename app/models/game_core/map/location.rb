module GameCore
  module Map
    class Location

      attr_reader :code_name, :x, :y

      @@locations = nil
      @@destinations = nil

      CITY_CLASS_NAME = 'GameCore::Map::City'
      WATER_CLASS_NAME = 'GameCore::Map::WaterArea'

      def initialize( code_name, data )
        @code_name = code_name
        @x = data[ :x ]
        @y = data[ :y ]
        @port = data[ :port ]
      end

      def port?()
        @port
      end

      def destinations
        Location.load_destinations
        result = []
        @@destinations[ @code_name ].each do |dest_code_name|
          result << Location.get_location( dest_code_name )
        end
        result
      end

      def destinations_codes_names
        Location.load_destinations
        @@destinations[ @code_name ]
      end

      def self.destinations_codes_names_from_code_name( code_name )
        code_name = load_destinations( code_name )
        @@destinations[ code_name ]
      end

      def id
        @code_name
      end

      # Get a Location object from a location_code_name
      #
      # @param [String] location_code_name the code name of the location you are seeking
      # @return [Location] the Location object
      def self.get_location( location_code_name )
        location = load_locations( location_code_name )
        @@locations[ location ][ :klass ].constantize.new( location, @@locations[ location ] )
      end

      def city?
        false
      end

      def water_area?
        false
      end

      private

      def self.assert_location( location )
        raise "Map::Location : Location not found #{location.inspect}" unless @@locations.has_key?( location )
      end

      def self.convert_location( location )
        if location
          location = location.to_sym
          assert_location( location )
        end
        location
      end

      def self.load_locations( location = nil )
        unless @@locations
          @@locations = YAML.load_file('app/models/game_core/map/data/locations.yml')
          @@locations.each_value do |v|
            v[ :klass ] = v[ :klass ] == :c ? CITY_CLASS_NAME : WATER_CLASS_NAME
          end
        end
        convert_location( location )
      end

      def self.load_destinations( location = nil )
        # Switch to yaml (from json)
        unless @@destinations
          @@destinations = YAML.load_file('app/models/game_core/map/data/destinations.yml')
        end
        load_locations( location )
      end

    end
  end
end
