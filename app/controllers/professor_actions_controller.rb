class ProfessorActionsController < ApplicationController

  def breed
    set_game_board

    monster = PMonster.find( params[ :monster_id ] )
    raise "Unable to find monster for #{params.inspect}" unless monster

    @game_board.p_professor.breed( monster )

    redirect_to g_game_board_play_url( g_game_board_id: @game_board.id )
  end

  def dont_move
    set_game_board

    prof_actions( nil )
  end

  def move
    set_game_board

    raise "Game board not in 'prof_move' state. Current state : : '#{@game_board.aasm_state}'" unless @game_board.aasm_state == 'prof_move'

    dest_loc = GameCore::Map::Location.get_location( params['zone_id'] )

    ActiveRecord::Base.transaction do
      @prof.prof_play( dest_loc )
    end

    redirect_to g_game_board_play_url( @game_board )

  end

  def invoke_nyog_sothep
    set_game_board

    @game_board.update( nyog_sothep_invoked: true, nyog_sothep_current_location_code_name: @game_board.nyog_sothep_invocation_position_code_name )
    GDestroyedCity.destroy_city( @game_board, @game_board.nyog_sothep_invocation_position_code_name )

    redirect_to g_game_board_play_url( g_game_board_id: @game_board.id )
  end

end
