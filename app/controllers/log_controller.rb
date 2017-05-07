class LogController < ApplicationController

  def list
    set_game_board

    @events = @game_board.e_event_logs.includes( :actor ).all.order( 'id DESC' )

  end

end
