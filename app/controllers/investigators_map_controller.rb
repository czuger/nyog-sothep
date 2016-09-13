require 'pp'

class InvestigatorsMapController < ApplicationController

  include GameLogic::InvestigatorsTurn

  def switch_table
    params[:event_table]
    set_current_investigator
    @current_investigator.update_attribute( :event_table, params[:event_table] )
    head :ok
  end

  def show

    @game_board = GGameBoard.first

    set_current_moving_investigator
    @zone = @current_investigator.current_location

    @events = @game_board.e_event_logs.all.order( 'logset DESC, id ASC' )

    @prof_positions = @game_board.p_prof_positions.all
    @monsters_positions = @game_board.p_monster_positions.all

    if @game_board.inv_move? && @current_investigator.ready?
      @aval_destinations = @zone.destinations
    end

  end

end
