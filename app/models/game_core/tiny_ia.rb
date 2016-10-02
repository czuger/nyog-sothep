module GameCore
  class TinyIa

    def self.professor_should_play?( game_board )
      ( game_board.players_count == 1 && game_board.ia_side == 'prof' ) &&
        ( game_board.prof_move? || game_board.prof_attack? || game_board.prof_fall_back? )
    end

    def self.professor_play( game_board )
      prof = game_board.p_professor
      professor_move( game_board, prof )
      professor_attack( game_board, prof )
      professor_breed( game_board, prof )
    end

    private

    def self.professor_attack( game_board, prof )
      choose, investigator = prof.prof_has_to_choose_attack?
      if choose && GameCore::Dices.d2 == 1
        prof.prof_fight( investigator )
      end

      # TODO : need to loop until all investigators foughts
      professor_move( game_board, prof ) if game_board.prof_fall_back?

      # TODO : need to loop until all investigators foughts
      game_board.prof_breed! if game_board.prof_attack?

    end

    def self.professor_breed( game_board, prof )

      prof_position = prof.current_location

      if prof_position.city? && prof.can_breed_in_city?( prof_position )
        fanatic = game_board.p_monsters.where( code_name: :fanatics ).first
        if fanatic
          prof.breed( fanatic )
        else
          if prof_position.city? && prof_position.port && ( deep_one = game_board.p_monsters.where( code_name: :profonds ).first )
            prof.breed( deep_one )
          else
            random_monster = game_board.p_monsters.where.not( code_name: :profonds ).to_a.sample
            prof.breed( random_monster ) if random_monster
          end
        end
      end
      game_board.inv_move! if game_board.prof_breed?
    end

    def self.professor_move( _, prof )
      dest = prof.current_location.destinations.sample
      prof.move( dest )
    end

  end
end
