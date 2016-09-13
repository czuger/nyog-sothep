require 'test_helper'

class ProfessorMapControllerTest < ActionDispatch::IntegrationTest

  test 'should show professor' do
    create( :g_game_board )
    get professor_map_show_url
    assert_response :success
  end

end
