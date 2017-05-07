require 'test_helper'

class LogControllerTest < ActionDispatch::IntegrationTest
  test "should get list" do
    get log_list_url
    assert_response :success
  end

end
