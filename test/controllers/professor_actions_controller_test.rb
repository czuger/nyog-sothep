require 'test_helper'

class ProfessorActionsControllerTest < ActionDispatch::IntegrationTest

  def setup
    @gb = create( :g_game_board_ready_for_fight )
    @professor = @gb.p_professor
    @dest = @professor.current_location.destinations.first
    @investigator = @gb.i_investigators.first
    @investigator.update( weapon: true )
  end

  test 'professor should fight and loose hard' do
    Kernel.stubs(:rand).returns(6)
    get attack_g_game_board_professor_actions_url( g_game_board_id: @gb.id, investigator_id: @investigator.id )
    assert_redirected_to g_game_board_play_url(  attacking_investigator_id: @investigator.id )
  end

  test 'professor should fight and loose' do
    Kernel.stubs(:rand).returns(3)
    get attack_g_game_board_professor_actions_url( g_game_board_id: @gb.id, investigator_id: @investigator.id )
    assert_redirected_to g_game_board_play_url( attacking_investigator_id: @investigator.id )
  end

  test 'professor should fight and nobody wins' do
    Kernel.stubs(:rand).returns(1)
    get attack_g_game_board_professor_actions_url( g_game_board_id: @gb.id, investigator_id: @investigator.id )
    assert_redirected_to g_game_board_play_url( attacking_investigator_id: @investigator.id )
  end

  test 'professor should fight and investigator is protected by sign' do
    @investigator.update( weapon: false, sign: true )
    get attack_g_game_board_professor_actions_url( g_game_board_id: @gb.id, investigator_id: @investigator.id )
    assert_redirected_to g_game_board_play_url( attacking_investigator_id: @investigator.id )
  end

  test 'professor should fight and investigator is crushed' do
    @investigator.update( weapon: false, sign: false )
    get attack_g_game_board_professor_actions_url( g_game_board_id: @gb.id, investigator_id: @investigator.id )
    assert_redirected_to g_game_board_play_url( attacking_investigator_id: @investigator.id )
  end

  test 'professor should move' do
    @gb.update( aasm_state: 'prof_move' )
    Kernel.stubs(:rand).returns(1)
    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: @dest.id, zone_class: @dest.class )
    assert_redirected_to g_game_board_play_url
  end

end
