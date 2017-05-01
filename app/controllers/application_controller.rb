class ApplicationController < ActionController::Base
  protect_from_forgery with: :exception

  private

  def set_game_board
    @game_board = GGameBoard.find( params[:g_game_board_id] )
    @prof = @game_board.p_professor
  end

  def set_position_x_decal
    @x_decal = 15
    @position_x_decal = {}
  end

  def assert( assertion, message = nil )
    raise "Assertion failed : #{message}" unless assertion
  end

end
