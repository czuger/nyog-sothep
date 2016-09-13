class ProfessorMapController < ApplicationController

  def show

    @game_board = GGameBoard.first

    @game_board.start_turn! if @game_board.start?

    @investigators = @game_board.i_investigators
    @prof = @game_board.p_professor
    @prof_location = @prof.current_location

    @monsters_positions = @game_board.p_monster_positions.all

    if @game_board.prof_move?
      @aval_destinations = @prof_location.destinations
      @prof_can_move = true
    end

  end
end
