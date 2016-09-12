require 'test_helper'

class MapControllerTest < ActionDispatch::IntegrationTest

  test 'should get show' do
    gb = create( :g_game_board )
    get g_game_board_maps_url( g_game_board_id: gb.id )
    assert_response :success
  end

end
