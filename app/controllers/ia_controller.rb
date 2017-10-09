class IaController < ApplicationController
  def show

    set_game_board
    gb_data = @game_board.ia_prof_position&.gb_data
    gb_data = [] unless gb_data

    @to_show_data = gb_data.map{ |k, v| v.merge( { turn: k } ) }
  end
end
