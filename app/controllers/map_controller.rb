require 'pp'

class MapController < ApplicationController

  def switch_table
    params[:event_table]
    set_current_investigator
    @current_investigator.update_attribute( :event_table, params[:event_table] )
    head :ok
  end

  def show

    # TODO : refaire sur papier les AASM pour la board et les invest.

    @game_board = GGameBoard.first

    @investigators = @game_board.i_investigators
    @events = @game_board.e_event_logs.all.order( 'logset DESC, id ASC' )
    @prof_maybe_positions = @game_board.p_prof_positions.all
    @prof = @game_board.p_professor
    @prof_location = @prof.current_location

    case @game_board.aasm_state
      when 'prof_move' then

        @monsters_positions = @game_board.p_monster_positions.all

        @aval_destinations = @prof_location.destinations
        @prof_move = true

      when 'inv_move' then

        @current_investigator = @game_board.i_investigators.where( aasm_state: :ready_to_move ).first
        if @current_investigator
          @zone = @current_investigator.current_location

          @monsters_positions = @game_board.p_monster_positions.where( discovered: true )

          if @current_investigator.ready_to_move?
            @aval_destinations = @zone.destinations
          end

          @prof_spotted = @prof.spotted

          @inv_move = true
        else
          @current_investigator = @game_board.i_investigators.where( aasm_state: [ :known_psy_help, :delayed ] ).first
          @inv_special_event = true if @current_investigator
        end

      when 'inv_event' then
        @inv_event = true

      else raise "Show case non implemented : #{@game_board.aasm_state}"
    end
  end
end
