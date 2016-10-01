require 'test_helper'

class MapControllerTest < ActionDispatch::IntegrationTest

  def setup
    @gb = create( :g_game_board )
  end

  test 'prof should skip next step as there is no investigator on the same city' do
    @gb.update( aasm_state: 'prof_attack' )
    get map_show_url
    assert_response :success
  end

  test 'inv should be repelled' do
    @gb.update( aasm_state: 'inv_repelled' )
    inv = create( :i_investigator, g_game_board_id: @gb.id )
    get map_show_url( attacking_investigator_id: inv.id )
    assert_response :success
  end

  test 'prof should fall back' do
    @gb.update( aasm_state: 'prof_fall_back' )
    get map_show_url
    assert_response :success
  end

  test 'should show professor' do
    get map_show_url
    assert_response :success
  end

  test 'should show professor breed screen' do
    @gb.update( aasm_state: 'prof_breed' )
    get map_show_url
    assert_response :success
  end

  test 'should show investigator' do
    gb = create( :g_game_board_with_event_ready_to_move_investigators )
    gb.update( aasm_state: :inv_move )
    get map_show_url
    assert_response :success
  end

end
