require 'test_helper'

class ActionsControllerTest < ActionDispatch::IntegrationTest

  test "should go psy" do
    gb = create( :g_game_board )
    patch go_psy_g_game_board_actions_url( g_game_board_id: gb.id )
    assert_redirected_to g_game_board_maps_url
  end

  test "should move" do
    gb = create( :g_game_board )
    dest = gb.i_investigators.first.current_location.destinations.first
    patch move_g_game_board_actions_url( g_game_board_id: gb.id, zone_id: dest.id, zone_class: dest.class )
    assert_redirected_to g_game_board_maps_url
  end

end
