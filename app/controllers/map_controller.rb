require 'pp'

class MapController < ApplicationController

  include GameLogic::ShowMap

  def play
    set_show_map_common_variables

    @game_over = @game_board.game_lost? || @game_board.game_won?

    if @game_board.prof_asked_for_fake_cities?

      @nb_cities = @game_board.asked_fake_cities_count
      @cities = GameCore::Map::City.all.reject{ |c| c.code_name == @prof_location.code_name }

    else
      # Regular prof play
      prof_play
    end

  end

  private

  def prof_play
    @prof_in_port = @prof_location.port?
    @prof_monsters = @game_board.p_monsters

    @aval_destinations = @prof_location.destinations

    @monster_at_prof_location = PMonsterPosition.where(
      g_game_board_id: @game_board.id, location_code_name: @prof_location.code_name ).exists?

    @nyog_sothep_invocation_possible = @game_board.check_nyog_sothep_invocation( @prof )
  end


end
