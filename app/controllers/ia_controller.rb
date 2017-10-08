class IaController < ApplicationController
  def show
    set_game_board
    @gb_data = @game_board.ia_prof_positions.gb_data
    @gb_data = [] unless @gb_data
  end
end
