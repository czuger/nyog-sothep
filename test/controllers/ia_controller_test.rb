require 'test_helper'

class IaControllerTest < ActionDispatch::IntegrationTest
  test "should get show" do
    get ia_show_url
    assert_response :success
  end

end
