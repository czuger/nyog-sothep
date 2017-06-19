module GameCore
  module Assertions

    private

    def assert_regular_movement_allowed( src, dest )
      unless src.destinations_codes_names.include?( dest.code_name )
        raise "Illegal move attempt : #{dest.code_name.inspect} not in #{src.destinations_codes_names}"
      end
    end

  end
end
