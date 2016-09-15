class ProfessorActionsController < ApplicationController

  def move
    set_game_board
    raise "Prof move called while game_board not in prof_move state : #{@game_board.inspect}" unless @game_board.prof_move?

    if params['zone_id'] && params['zone_class']
      dest_loc = params['zone_class'].constantize.find( params['zone_id'] )
    else
      raise "Zone error : #{params.inspect}"
    end

    @game_board = GGameBoard.find( params[:g_game_board_id])

    ActiveRecord::Base.transaction do
      @game_board.p_professor.current_location = dest_loc
      @game_board.p_professor.save!
      @game_board.prof_move_end!
    end

    redirect_to map_show_url
  end

end
