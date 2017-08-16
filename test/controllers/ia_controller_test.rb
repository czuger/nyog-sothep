require 'test_helper'

class IaControllerTest < ActionDispatch::IntegrationTest
  test "should get show" do
    @gb = create( :g_game_board )
    get ia_show_g_game_board_professor_actions_url( @gb )
    assert_response :success
  end

end
