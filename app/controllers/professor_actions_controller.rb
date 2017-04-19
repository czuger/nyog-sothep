class ProfessorActionsController < ApplicationController

  def attack
    set_game_board
    investigator = @game_board.i_investigators.find( params[ :investigator_id ] )

    @game_board.p_professor.prof_fight( investigator )

    redirect_to g_game_board_play_url( g_game_board_id: @game_board.id, attacking_investigator_id: params[ :investigator_id ] )
  end

  def move
    set_game_board

    if params['zone_id'] && params['zone_class']
      dest_loc = params['zone_class'].constantize.find( params['zone_id'] )
    else
      raise "Zone error : #{params.inspect}"
    end

    # Prof move
    @prof.move( dest_loc )

    # Prof get a monster if count less than 4
    if @game_board.p_monsters.count < 4
      @prof.pick_one_monster
    end

    @game_board.prof_movement_done!

    # At the end of the professor move, investigators play
    GameCore::InvestigatorsActions.new( @game_board ).investigators_ia_play

    check_prof_asked_for_fake_cities{ redirect_to g_game_board_play_url( @game_board ) }
  end

end
