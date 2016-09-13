class ProfessorMapController < ApplicationController

  def show

    @game_board = GGameBoard.find( params[:g_game_board_id] )

    @game_board.start_turn! if @game_board.start?

    if @game_board.prof_move

      @investigators = @game_board.i_investigators
      @prof = @game_board.p_professor
      @prof_location = @prof.current_location

      @monsters_positions = @game_board.p_monster_positions.all

      @aval_destinations = @prof_location.destinations

    else
      @prof_waiting = true
    end

  end
end
