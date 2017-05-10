require 'test_helper'

class LogControllerTest < ActionDispatch::IntegrationTest
  test "should get list" do
    get g_game_board_log_list_url( create( :g_game_board ) )
    assert_response :success
  end

end
