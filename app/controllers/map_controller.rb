require 'pp'

class MapController < ApplicationController

  include GameLogic::GameBoardStatusRedirection
  include GameLogic::ShowMap

  def show
    set_show_map_common_variables

    check_prof_asked_for_fake_cities{}

    @prof_in_port = @prof_location.port?
    @prof_monsters = @game_board.p_monsters

    @aval_destinations = @prof_location.destinations

    @monster_at_prof_location = PMonsterPosition.where(
      g_game_board_id: @game_board.id, location_code_name: @prof_location.code_name ).exists?

    @nyog_sothep_invocation_possible = @game_board.check_nyog_sothep_invocation( @prof )
  end

end
