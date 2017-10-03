require 'test_helper'

class LogControllerTest < ActionDispatch::IntegrationTest

  test "should get list" do
    gb = create( :g_game_board )
    inv = create( :i_investigator, g_game_board: gb )
    create( :l_log, g_game_board: gb, actor: inv )

    get g_game_board_log_index_url( gb )
    assert_response :success
  end

end
