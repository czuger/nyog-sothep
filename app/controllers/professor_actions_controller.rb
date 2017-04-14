class ProfessorActionsController < ApplicationController

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

    # Prof move
    @prof.move( dest_loc )

    # Prof get a monster if count less than 4
    if @game_board.p_monsters.count < 4
      @prof.pick_one_monster
    end

    # Investigators play
    @game_board.alive_investigators.each do |i|
      i.ia_play( @game_board, @prof )
    end

    redirect_to g_game_board_play_url( g_game_board_id: @game_board.id )
  end

end
