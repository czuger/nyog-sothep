module GameCore
  module ProfessorFight

    # Check if a fight can happen.
    #
    # @param [GGameBoard] game_board the current game board
    def check_for_investigators_to_fight_in_city( game_board )
      investigators = game_board.alive_investigators.order( :id ).select{ |i| i.current_location_code_name == current_location_code_name }

      unless investigators.empty?
        fight_occurs = false
        investigators.each do |i|
          # On prof turn, we attack only if investigator does not have a weapon, or a medaillon
          unless i.weapon || i.medaillon
            fight( game_board, i )
            fight_occurs = true
          end
        end

        if fight_occurs
          # Si l'on a combatu, on sait ou est le professeur
          spotted( game_board )
        else
          # Sinon il est a l'endroit d'un des investigateurs
          trust_value = 1.0 / investigators.count
          investigators.each do |i|
            IInvTargetPosition.find_or_create_by!( g_game_board_id: game_board.id, position_code_name: i.current_location_code_name, trust: trust_value, turn: game_board.turn )
          end
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
              LLog.log( gb, investigator, 'log.prof_missed' )
            else
              raise "Quantic perturbation : roll = #{result}"
          end
          # gb.prof_fall_back!

        elsif investigator.sign
          investigator.loose_san( gb, 2 )
          investigator.update( sign: false )
          LLog.log( gb, investigator, 'prof_fight.sign_protect', true,
                    { san_loss: 2, cur_san: investigator.san },  )
          # gb.inv_repelled!

        else
          investigator.loose_san( gb, 4 )
          LLog.log( gb, investigator, 'prof_fight.no_protection', true,
                    { san_loss: 4, cur_san: investigator.san  } )
          # gb.inv_repelled!

        end
      end

    end
  end
end
