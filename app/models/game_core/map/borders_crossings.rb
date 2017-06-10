module GameCore
  module Map
    class BordersCrossings < GameCore::Map::Location

      @@borders_crossings = nil

      def self.check?( src_code_name, dest_code_name )
        load_border_crossings

        src_code_name = src_code_name.to_sym
        dest_code_name = dest_code_name.to_sym

        crossings = @@borders_crossings[ src_code_name ]

        return true if crossings && crossings.include?( dest_code_name )

        false
      end

      private

      def self.load_border_crossings
        unless @@borders_crossings
          @@borders_crossings = YAML.load_file('app/models/game_core/map/data/borders_crossings.yml')
        end
      end

    end
  end
end
