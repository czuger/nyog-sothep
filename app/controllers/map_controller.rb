require 'pp'

class MapController < ApplicationController

  include GameLogic::GameBoardStatusRedirection

  def show
    set_game_board

    @investigators = @game_board.alive_investigators
    @events = @game_board.e_event_logs.all.order( 'logset DESC, id ASC' )

    @prof_location = @prof.current_location

    @nyog_sothep_location = @game_board.nyog_sothep_invocation_position
    @nyog_sothep_location_rotation = @game_board.nyog_sothep_invocation_position_rotation

    @prof_in_port = @prof_location.port

    @prof_monsters = @game_board.p_monsters
    @monsters_positions = @game_board.p_monster_positions.all

    @aval_destinations = @prof_location.destinations

    @monster_at_prof_location = PMonsterPosition.where(
      g_game_board_id: @game_board.id, location_type: @prof_location.class.to_s, location_id: @prof_location.id ).exists?

    if @game_board.inv_move? || @game_board.inv_events?
      redirect_to g_game_board_investigators_ia_play_url( @game_board )
    end

    check_prof_asked_for_fake_cities{}
  end

end
