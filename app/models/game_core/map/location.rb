module GameCore
  module Map
    class Location

      @@locations = nil
      @@destinations = nil

      def initialize( code_name )
        @code_name = code_name
      end

      def port?()
        @@locations[ @code_name ][ :port ]
      end

      def destinations
        result = []
        @@destinations.each do |k, dest|
          dest.each{ |dest_str| result << Location.get_location( dest_str ) }
        end
        result
      end

      def id
        @code_name
      end

      def self.get_location( location )
        self.load_data
        location = location.to_sym
        assert_location( location )
        @@locations[ location ][ :klass ].constantize.new( location )
      end

      private

      def self.assert_location( location )
        raise "Map::Location : Location not found #{location.inspect}" unless @@locations.has_key?( location )
      end

      def self.load_data
        unless @@locations
          file = File.read('app/models/game_core/map/locations.json')
          @@locations = JSON.parse(file)
          @@locations.symbolize_keys!
          @@locations.each_value{ |h| h.symbolize_keys! }
        end
        unless @@destinations
          file = File.read('app/models/game_core/map/destinations.json')
          @@destinations = JSON.parse(file)
          @@destinations.symbolize_keys!
        end
      end

    end
  end
end
