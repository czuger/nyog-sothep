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
        result = []
        @@destinations[ @code_name ].each do |dest_code_name|
          result << Location.get_location( dest_code_name )
        end
        result
      end

      def destinations_codes_names
        @@destinations[ @code_name ]
      end

      def self.destinations_codes_names_from_code_name( code_name )
        load_data
        assert_location( code_name )
        @@destinations[ code_name ]
      end

      def id
        @code_name
      end

      def self.get_location( location )
        self.load_data
        location = location.to_sym
        assert_location( location )
        @@locations[ location ][ :klass ].constantize.new( location, @@locations[ location ] )
      end

      private

      def self.assert_location( location )
        raise "Map::Location : Location not found #{location.inspect}" unless @@locations.has_key?( location )
      end

      def self.load_data
        unless @@locations
          @@locations = YAML.load_file('app/models/game_core/map/locations.yml')
          @@locations.each_value do |v|
            v[ :klass ] = v[ :klass ] == :c ? CITY_CLASS_NAME : WATER_CLASS_NAME
          end
        end
        unless @@destinations
          file = File.read('app/models/game_core/map/destinations.json')
          @@destinations = JSON.parse(file)
          @@destinations.symbolize_keys!
          @@destinations.each_key do |k|
            @@destinations[ k ] = @@destinations[ k ].map{ |e| e.to_sym }
          end
        end
      end

    end
  end
end
