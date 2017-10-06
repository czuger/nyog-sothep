class LogController < ApplicationController

  def index
    set_game_board

    @events = @game_board.l_logs.paginate(:page => params[:page], :per_page => 40)
  end

  def show
    @log = LLog.find( params[ :id ] )
  end

end
