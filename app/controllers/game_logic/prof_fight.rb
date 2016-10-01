module GameLogic
  module ProfFight

    def prof_fight( investigator, professor )
      if investigator.weapon
        result = GameCore::Dices.d6
        case result
          when 6
            professor.loose_life( @game_board, investigator, 3 )
          when 2..5
            professor.loose_life( @game_board, investigator, 2 )
          when 1
            EEventLog.log( @game_board, I18n.t( 'prof_fight.gun_miss', investigator_name: investigator.translated_name ) )
          else
            raise "Quantic perturbation : roll = #{result}"
        end
        @game_board.prof_fall_back!

      elsif investigator.sign
        EEventLog.log( @game_board, I18n.t( 'prof_fight.sign_protect', investigator_name: investigator.translated_name ) )
        investigator.loose_san( @game_board, 2 )
        @game_board.inv_repelled!

      else
        EEventLog.log( @game_board, I18n.t( 'prof_fight.no_protection', investigator_name: investigator.translated_name ) )
        investigator.loose_san( @game_board, 4 )
        @game_board.inv_repelled!

      end
    end
  end
end
