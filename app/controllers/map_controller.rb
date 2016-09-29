require 'pp'

class MapController < ApplicationController

  include GameLogic::Events
  include GameLogic::BreedCheck

  def show
    @game_board = GGameBoard.first

    @investigators = @game_board.i_investigators
    @events = @game_board.e_event_logs.all.order( 'logset DESC, id ASC' )
    @prof_maybe_positions = @game_board.p_prof_positions.all
    @prof = @game_board.p_professor
    @prof_location = @prof.current_location

    begin
      @loop = false
      case @game_board.aasm_state
        when 'prof_move' then
          prof_move
        when 'prof_breed' then
          prof_breed
        when 'inv_move' then
          inv_move
        when 'inv_event' then
          process_events

        else raise "Show case non implemented : #{@game_board.aasm_state}"
      end
    end while @loop

  end

  private

  def inv_move
    if( @current_investigator = @game_board.next_moving_investigator )
      @zone = @current_investigator.current_location
      @monsters_positions = @game_board.p_monster_positions.where( discovered: true )
      @aval_destinations = @zone.destinations
      @prof_spotted = @prof.spotted
      @inv_move = true
    else
      @game_board.inv_move_end!
      @loop = true
    end
  end

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

    prof_current_location = @prof.current_location
    @prof_in_water = !prof_current_location.city?
    @prof_in_port = prof_current_location.port if prof_current_location.city?

    @prof_breed = true if can_breed_in_city?( prof_current_location )
  end
end
