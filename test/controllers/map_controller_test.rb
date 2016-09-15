require 'test_helper'

class MapControllerTest < ActionDispatch::IntegrationTest

  test 'should show professor' do
    create( :g_game_board )
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
