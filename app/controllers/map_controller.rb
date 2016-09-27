require 'pp'

class MapController < ApplicationController

  include GameLogic::Events

  def switch_table
    params[:event_table]
    set_current_investigator
    @current_investigator.update_attribute( :event_table, params[:event_table] )
    head :ok
  end

  def show
    @game_board = GGameBoard.first

    @investigators = @game_board.i_investigators
    @events = @game_board.e_event_logs.all.order( 'logset DESC, id ASC' )
    @prof_maybe_positions = @game_board.p_prof_positions.all
    @prof = @game_board.p_professor
    @prof_location = @prof.current_location

    replay = nil
    begin
      replay = false
      case @game_board.aasm_state
        when 'prof_move' then
          prof_move

          # TODO : prof should not be able to breed on an occuped position.
        when 'prof_breed' then
          prof_breed

        when 'inv_move' then

          # puts "Next moving investigator = #{@game_board.next_moving_investigator.inspect}"

          if( @current_investigator = @game_board.next_moving_investigator )
            @zone = @current_investigator.current_location
            @monsters_positions = @game_board.p_monster_positions.where( discovered: true )
            @aval_destinations = @zone.destinations
            @prof_spotted = @prof.spotted
            @inv_move = true
          else
            @game_board.inv_move_end!
            replay = process_events
          end

        when 'inv_event' then
          replay = process_events

        else raise "Show case non implemented : #{@game_board.aasm_state}"
      end
    end while replay

    prof_move if @game_board.prof_move?

  end

  private

  def prof_move
    unless @prof_move
      @monsters_positions = @game_board.p_monster_positions.all
      @prof_monsters = @game_board.p_monsters

      @aval_destinations = @prof_location.destinations
      @prof_move = true
    end
  end

  def prof_breed
    @monsters_positions = @game_board.p_monster_positions.all
    @prof_monsters = @game_board.p_monsters

    @prof_breed = true
  end
end
