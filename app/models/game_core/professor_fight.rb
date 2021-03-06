module GameCore
  module ProfessorFight

    # Check if a fight can happen.
    #
    # @param [GGameBoard] game_board the current game board
    def check_for_investigators_to_fight_in_city( game_board )

      # We fight only in cities
      if GameCore::Map::Location.get_location( current_location_code_name ).city?
        investigators = game_board.alive_investigators.order( :id ).select{ |i| i.current_location_code_name == current_location_code_name }

        unless investigators.empty?

          prof_position_finder = GameCore::Ia::ProfPositionFinder.new
          prof_position_finder.load( game_board )

          investigators.each do |inv|
            # On prof turn, we attack only if investigator does not have a weapon, or a medaillon
            unless inv.weapon
              fight( game_board, inv )
              spotted( game_board, inv, prof_position_finder )
            end
          end

          prof_position_finder.save( game_board )

        end
      end
    end

    def fight( game_board, investigator )

      # TODO : check that investigator and prof are at the same place
      # raise "Investigator and prof not at the same place : #{investigator}"

      gb = game_board

      ActiveRecord::Base.transaction do

        if investigator.weapon
          result = GameCore::Dices.d6
          case result
            when 6
              loose_life( gb, investigator, 3 )
            when 2..5
              loose_life( gb, investigator, 2 )
            when 1
              LLog.log( gb, investigator, 'fight.gun_miss' )
            else
              raise "Quantic perturbation : roll = #{result}"
          end
          # gb.prof_fall_back!

        elsif investigator.sign
          investigator.loose_san( gb, 2 )
          investigator.update( sign: false )
          LLog.log( gb, investigator, 'fight.sign_protect', true,
                    { san_loss: 2, cur_san: investigator.san }, true  )
          # gb.inv_repelled!

        else
          investigator.loose_san( gb, 4 )
          LLog.log( gb, investigator, 'fight.no_protection', true,
                    { san_loss: 4, cur_san: investigator.san  } )
          # gb.inv_repelled!

        end
      end

    end
  end
end
