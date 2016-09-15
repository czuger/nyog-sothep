class ApplicationController < ActionController::Base
  protect_from_forgery with: :exception

  private

  def set_game_board
    @game_board = GGameBoard.find( params[:g_game_board_id])
  end

  def set_current_investigator
    @current_investigator = IInvestigator.find( params[:id])
  end

  def investigators_ready_to_move
    @game_board.i_investigators.where( aasm_state: :ready_to_move ).order( :id )
  end

end
