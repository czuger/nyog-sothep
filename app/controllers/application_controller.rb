class ApplicationController < ActionController::Base
  protect_from_forgery with: :exception

  private

  def set_game_board
    @game_board = GGameBoard.find( params[:g_game_board_id] )
  end

  def set_current_investigator
    @current_investigator = IInvestigator.find( params[:id])
  end

end
