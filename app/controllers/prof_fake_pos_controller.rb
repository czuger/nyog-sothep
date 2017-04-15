class ProfFakePosController < ApplicationController

  def new
    set_game_board

    @investigators = @game_board.alive_investigators
    @prof_location = @prof.current_location
    @monsters_positions = @game_board.p_monster_positions.all
    @nb_cities = params[ :nb_cities ].to_i

    @cities = CCity.all.reject{ |e| e == @prof_location }
  end

  def create
  end
end
