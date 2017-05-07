class LogController < ApplicationController

  def list
    set_game_board

    @events = @game_board.e_event_logs.includes( :actor ).order( 'id DESC' ).paginate(:page => params[:page], :per_page => 20)

  end

end
