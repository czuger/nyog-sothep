module GameCore
  module Ia
    module InvestigatorMovementTarget

      # exclusion_city_code_name : a list of all unavaliable city (destroyed, forbiden, etc ...). An array of strings
      def select_new_target( game_board, exclusion_city_code_name )

        # prof_position is an array of [ [ 'oxford', 0.9 ], 'lowel', 0.8 ], etc ... ]
        prof_positions = game_board.i_inv_target_positions.order( 'trust DESC' ).pluck( :position_code_name, :trust )
        prof_positions.reject!{ |e| exclusion_city_code_name.include?( e[0] ) }

        if weapon
          # If we have a weapon, then we chase the prof
          target_position_code_name = guess_prof_location( game_board, prof_positions )
          puts "#{translated_name} now change target to chase the professor at #{target_position_code_name}" if IInvestigator::DEBUG_MOVEMENTS

          target_position_code_name = choose_random_location( exclusion_city_code_name ) unless target_position_code_name
        else
          # Otherwise we flee from him
          target_position_code_name = guess_prof_location( game_board, prof_positions )
          if target_position_code_name
            #

          else
            # In this case we don't know where the prof is, so we walk randomly
            target_position_code_name = choose_random_location( exclusion_city_code_name )
          end
        end

        target_position_code_name
      end

      private

      def choose_random_location( exclusion_city_code_name )
        exclusion_list = exclusion_city_code_name + [ current_location_code_name, last_location_code_name, ia_target_destination_code_name ]
        target_position_code_name = GameCore::Map::City.random_city_code_name( exclusion_list.map( &:to_sym ) )
        raise "Can't find a random position : exclusion_list = #{exclusion_list.inspect}" unless target_position_code_name
        puts "#{translated_name} now change target randomly at #{target_position_code_name}" if IInvestigator::DEBUG_MOVEMENTS
        target_position_code_name
      end

      def guess_prof_location( game_board, prof_positions )
        first_prof_position = prof_positions.first
        # If we know where the prof is, every chasing investigator go there
        return first_prof_position[0] if first_prof_position[1] >= 0.9

        # So here we don't have a sure prof position, we will guess it
        investigators_targets = game_board.i_investigator.pluck( :ia_target_destination_code_name )
        # In fact we send investigators everywhere we have information
        prof_positions.each do |pp|
          # If nobody already chasing the prof on that pos, then we go there
          return pp[0] unless investigators_targets.include?( pp[0] )
        end

        # If all slots are filled, then choose randomly
        random_pp = prof_positions.sample
        random_pp.first if random_pp
      end

    end
  end
end
