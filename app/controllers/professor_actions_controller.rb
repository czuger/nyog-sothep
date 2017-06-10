class ProfessorActionsController < ApplicationController

  include GameLogic::GameBoardStatusRedirection

  def breed
    set_game_board

    monster = PMonster.find( params[ :monster_id ] )
    raise "Unable to find monster for #{params.inspect}" unless monster

    @game_board.p_professor.breed( monster )

    redirect_to g_game_board_play_url( g_game_board_id: @game_board.id )
  end

  def move
    set_game_board

    raise "Game board not in 'prof_move' state. Current state : : '#{@game_board.aasm_state}'" unless @game_board.aasm_state == 'prof_move'

    # TODO : remove zone_class from parameters (awfull)
    # if params['zone_id'] && params['zone_class'] && (params['zone_class'] == 'CCity' || params['zone_class'] == 'WWaterArea')
    #   dest_loc = params['zone_class'].constantize.find( params['zone_id'] )
    # else
    #   raise "Zone error : #{params.inspect}"
    # end

    dest_loc = GameCore::Map.Location.get_location( params['zone_id'] )

    ActiveRecord::Base.transaction do
      # Prof move
      @prof.move( dest_loc )

      @prof.check_for_investigators_to_fight_in_city( @game_board )

      # Prof get a monster if count less than 4
      if @game_board.p_monsters.count < 5
        @prof.pick_one_monster
      end

      @game_board.prof_movement_done!

      # At the end of the professor move, investigators play
      @game_board.investigators_ia_play( @prof )
    end

    check_prof_asked_for_fake_cities{ redirect_to g_game_board_play_url( @game_board ) }
  end

end
