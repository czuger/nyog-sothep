module GameCore
  module Ia

    class BfsAlgo

      def self.find_next_dest_to_goal( current_position, goal, forbidden_city: nil )

        raise "Current position and goal are the same" if current_position == goal

        goal_name = goal.code_name

        frontier = []
        frontier << current_position.code_name
        came_from = {}
        came_from[ current_position.code_name ] = nil

        until frontier.empty?
          current_name = frontier.shift

          if current_name == goal_name
            break
          end

          DESTINATIONS[ current_name ].each do |next_location|
            next if next_location == forbidden_city&.code_name
            unless came_from.has_key?( next_location )
              frontier << next_location
              came_from[ next_location ] = current_name
            end
          end
        end

        back_token = goal_name
        next_step = nil

        while back_token != current_position.code_name
          next_step = back_token
          back_token = came_from[ back_token ]
          # p next_step
          break if next_step == nil
        end

        DEST_KLASS[ next_step ].constantize.find_by( code_name: next_step )
      end

      def self.create_dest_hash

        dest_hash = {}
        class_hash = {}

        CCity.all.each do |c|
          dest_hash[ c.code_name ] = c.destinations.map{ |e| e.code_name }
          class_hash[ c.code_name ] = c.class.to_s
        end

        WWaterArea.all.each do |c|
          dest_hash[ c.code_name ] = c.destinations.map{ |e| e.code_name }
          class_hash[ c.code_name ] = c.class.to_s
        end

        pp dest_hash
        pp class_hash

      end
    end
  end
end
