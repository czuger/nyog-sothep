require 'test_helper'

class MapControllerTest < ActionDispatch::IntegrationTest

  def setup
    @gb = create( :g_game_board )
  end

  test 'prof ia should play' do
    gb = create( :g_game_board_ready_for_ia_play )
    gb.reload
    get g_game_board_play_url( g_game_board_id: gb.id )
    assert_response :success
  end

  test 'prof should skip next step as there is no investigator on the same city' do
    @gb.update( aasm_state: 'prof_attack' )
    get g_game_board_play_url( g_game_board_id: @gb.id )
    assert_response :success
  end

  test 'inv should be repelled' do
    @gb.update( aasm_state: 'inv_repelled' )
    inv = create( :i_investigator, g_game_board_id: @gb.id )
    get g_game_board_play_url( g_game_board_id: @gb.id, attacking_investigator_id: inv.id )
    assert_response :success
  end

  test 'prof should fall back' do
    @gb.update( aasm_state: 'prof_fall_back' )
    get g_game_board_play_url( g_game_board_id: @gb.id )
    assert_response :success
  end

  test 'should show professor' do
    get g_game_board_play_url( g_game_board_id: @gb.id )
    assert_response :success
  end

  test 'should show professor breed screen' do
    @gb.update( aasm_state: 'prof_breed' )
    get g_game_board_play_url( g_game_board_id: @gb.id )
    assert_response :success
  end

  test 'should show investigator' do
    gb = create( :g_game_board_with_event_ready_to_move_investigators )
    gb.update( aasm_state: :inv_move )
    get g_game_board_play_url( g_game_board_id: @gb.id )
    assert_response :success
  end

  test 'should show monsters' do
    create :fanatiques, g_game_board_id: @gb.id, location_code_name: :oxford
    get g_game_board_play_url( g_game_board_id: @gb.id )
    assert_response :success
  end

end
