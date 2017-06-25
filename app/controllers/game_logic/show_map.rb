module GameLogic
  module ShowMap

    def set_show_map_common_variables
      set_game_board

      @investigators = @game_board.alive_investigators

      @nyog_sothep_location = @game_board.nyog_sothep_position
      @nyog_sothep_location_rotation = @game_board.nyog_sothep_invocation_position_rotation

      @prof_location = @prof.current_location

      @monsters_positions = @game_board.p_monster_positions.all

      @destroyed_cities = @game_board.g_destroyed_cities

      @log_summaries = @game_board.e_event_log_summaries.limit( 30 )

      set_position_x_decal
    end

  end
end
