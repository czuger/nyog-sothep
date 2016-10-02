module GameLogic
  module TinyIa

    def tiny_ia_professor_move( game_board )

      if game_board.players_count == 1 && game_board.ia_side == 'prof'
        dest = game_board.p_professor.current_location.destinations.sample
        redirect_to move_g_game_board_professor_actions_url( g_game_board_id: game_board.id, zone_id: dest.id, zone_class: dest.class )
      end
    end

  end
end
