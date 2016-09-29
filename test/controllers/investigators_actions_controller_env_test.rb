require 'test_helper'

class InvestigatorsActionsControllerEnvTest < ActionDispatch::IntegrationTest

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @investigator = @gb.i_investigators.first
  end

  test 'investigator should replay then should be able to move again' do

    GameCore::Events.stubs(:event_dices).returns(9) # Should fall in replay case

    get map_show_url

    @dest = @investigator.current_location.destinations.first
    get move_g_game_board_investigators_action_url( g_game_board_id: @gb.id, id: @investigator.id, zone_id: @dest.id, zone_class: @dest.class )
    assert_redirected_to map_show_url
    assert_equal @dest, @investigator.reload.current_location
    assert @gb.inv_event?
  end

end
