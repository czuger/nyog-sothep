class InvestigatorsActionsController < ApplicationController

  include GameCore::Movement

  def switch_table
    params[:event_table]
    set_current_investigator
    @current_investigator.update_attribute( :event_table, params[:event_table] )
    head :ok
  end

  def move
    set_game_board

    raise "Investigator move called while game_board not in inv_move state : #{@game_board.inspect}" unless @game_board.inv_move?

    if params['zone_id'] && params['zone_class']
      dest_loc = params['zone_class'].constantize.find( params['zone_id'] )
    else
      raise "Movement destinatin not set : #{params.inspect}"
    end

    ActiveRecord::Base.transaction do

      # First step : pass all investigators that does not move this turn
      # @game_board.investigators_that_pass_move_phase.each do |inv|
      #   EEventLog.start_event_block( @game_board )
      #   inv.no_move_to_event_step!
      # end

      EEventLog.start_event_block( @game_board )
      set_current_investigator

      unless @current_investigator.inv_move?
        raise "Investigator able to move only if ready : #{@current_investigator.inspect}"
      end

      if regular_move_token( @game_board, @current_investigator, dest_loc )
        @current_investigator.roll_event!
      else
        @current_investigator.roll_no_event!
      end

#       inv_move_end

      # check_end_of_movements
    end

    redirect_to g_game_board_play_url( g_game_board_id: @game_board.id )
  end

  private

  # def check_end_of_movements
  #   # If all the investigators have moved, we set the game board on next state
  #   unless @game_board.get_next_investigator_for_move_phase
  #     @game_board.inv_move_end!
  #   end
  #
  # end

end
