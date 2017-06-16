class LogController < ApplicationController

  def index
    set_game_board

    @events = @game_board.e_event_logs.paginate(:page => params[:page], :per_page => 20)
  end

  def show
    @log = EEventLog.find( params[ :id ] )
  end

end
