class ProfessorActionsController < ApplicationController

  include GameLogic::BreedCheck
  include GameLogic::ProfFight

  def attack
    set_game_board
    investigator = @game_board.i_investigators.find( params[ :investigator_id ] )

    @game_board.p_professor.prof_fight( investigator )

    redirect_to g_game_board_play_url( g_game_board_id: @game_board.id, attacking_investigator_id: params[ :investigator_id ] )
  end

  def dont_attack
    set_game_board

    EEventLog.start_event_block( @game_board )
    EEventLog.log( @game_board, I18n.t( 'log.prof_in_inv_city' ) )

    @game_board.prof_breed!
    redirect_to g_game_board_play_url( g_game_board_id: @game_board.id )
  end

  def dont_breed
    set_game_board
    @game_board.inv_move!
    redirect_to g_game_board_play_url( g_game_board_id: @game_board.id )
  end

  def breed
    set_game_board

    monster = PMonster.find( params[ :monster_id ] )
    raise "Unable to find monster for #{params.inspect}" unless monster

    @game_board.p_professor.breed( monster )

    redirect_to g_game_board_play_url( g_game_board_id: @game_board.id )
  end

  def move
    set_game_board

    if params['zone_id'] && params['zone_class']
      dest_loc = params['zone_class'].constantize.find( params['zone_id'] )
    else
      raise "Zone error : #{params.inspect}"
    end

    @game_board.p_professor.move( dest_loc )

    redirect_to g_game_board_play_url( g_game_board_id: @game_board.id )
  end

end
