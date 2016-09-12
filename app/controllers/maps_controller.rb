require 'pp'

class MapsController < ApplicationController

  include GameLogic::Turn

  def switch_table
    params[:event_table]
    set_current_investigator
    @current_investigator.update_attribute( :event_table, params[:event_table] )
    head :ok
  end

  def show

    @game_board = GGameBoard.find( params[:g_game_board_id])

    set_current_investigator
    @zone = @current_investigator.current_location

    @events = @game_board.e_event_logs.all.order( 'logset DESC, id ASC' )

    @prof = @game_board.p_professor
    @prof_zone = @prof.current_location

    @prof_positions = @game_board.p_prof_positions.all
    @monsters_positions = @game_board.p_monster_positions.all

    @aval_destinations = @zone.destinations

  end

end
