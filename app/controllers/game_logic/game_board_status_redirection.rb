module GameLogic

  # This module handle redirection according to game status
  module GameBoardStatusRedirection

    private

    def check_prof_asked_for_fake_cities
      if @game_board.prof_asked_for_fake_cities?
        redirect_to new_g_game_board_prof_fake_pos_url( g_game_board_id: @game_board.id, nb_cities: @game_board.asked_fake_cities_count )
      else
        yield
      end
    end

  end
end
