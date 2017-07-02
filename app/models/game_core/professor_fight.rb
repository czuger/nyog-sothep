module GameCore
  module ProfessorFight

    def check_for_investigators_to_fight_in_city( game_board )
      investigators = game_board.alive_investigators.order( :id ).select{ |i| i.current_location_code_name == current_location_code_name }

      unless investigators.empty?
        fight_occurs = false
        investigators.each do |i|
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
          investigators.each do |i|
            IInvTargetPosition.find_or_create_by!( g_game_board_id: game_board.id, position_code_name: i.current_location_code_name, memory_counter: 5 )
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
              LLog.log( gb, investigator, 'prof_fight.gun_miss', {} )
            else
              raise "Quantic perturbation : roll = #{result}"
          end
          # gb.prof_fall_back!

        elsif investigator.sign
          LLog.log( gb, investigator, 'prof_fight.sign_protect', {} )
          investigator.loose_san( gb, 2 )
          investigator.update( sign: false )
          # gb.inv_repelled!

        else
          LLog.log( gb, investigator, 'prof_fight.no_protection', {} )
          investigator.loose_san( gb, 4 )
          # gb.inv_repelled!

        end
      end

    end
  end
end
