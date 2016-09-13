require 'test_helper'

class InvestigatorsMapControllerTest < ActionDispatch::IntegrationTest

  test 'should show investigators' do
    create( :g_game_board )
    get investigators_map_show_url
    assert_response :success
  end

end
