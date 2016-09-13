class ProfessorActionsController < ApplicationController

  def move

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
      @game_board.prof_attack_end!    # We will implement prof attack later
      EEventLog.start_event_block( @game_board )
    end

    redirect_to professor_map_show_url

  end

end
