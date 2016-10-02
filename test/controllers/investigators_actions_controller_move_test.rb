require 'test_helper'

class InvestigatorsActionsControllerMoveTest < ActionDispatch::IntegrationTest

  def setup
    @gb = create( :g_game_board_with_event_ready_to_move_investigators )
    @investigator = @gb.i_investigators.first
    @dest = @investigator.current_location.destinations.first
  end

  test 'switch event table' do
    post switch_table_g_game_board_investigators_action_url( g_game_board_id: @gb.id, id: @investigator.id, event_table: 2 )
  end

  test "should go psy" do
    get go_psy_g_game_board_investigators_action_url( g_game_board_id: @gb.id, id: @investigator.id )
    assert_redirected_to g_game_board_play_url( g_game_board_id: @gb.id )
  end

  test "investigator should move" do
    get move_g_game_board_investigators_action_url( g_game_board_id: @gb.id, id: @investigator.id, zone_id: @dest.id, zone_class: @dest.class )
    assert_redirected_to g_game_board_play_url( g_game_board_id: @gb.id )
    assert @gb.inv_move?
  end

  test "investigator should through a border and fail" do
    border_cross_road = create( :inv_cross_border_road )
    @investigator.current_location = border_cross_road.src_city
    @investigator.save!
    @dest = border_cross_road.dest_city
    Kernel.stubs(:rand).returns(6) # Stubs the dice method in order it fails always

    get move_g_game_board_investigators_action_url( g_game_board_id: @gb.id, id: @investigator.id, zone_id: @dest.id, zone_class: @dest.class )
    assert_redirected_to g_game_board_play_url( g_game_board_id: @gb.id )
    assert_equal border_cross_road.src_city, @investigator.reload.current_location
    assert @gb.inv_move?
  end

  # test "should roll events for investigators on table 1" do
  #   @gb = create( :g_game_board_with_event_ready_for_events_investigators )
  #   @gb.prof_move_end!
  #   @gb.inv_move_end!
  #   InvestigatorsActionsController.any_instance.stubs(:d6).returns(1)
  #   get g_game_board_investigators_actions_roll_events_url( g_game_board_id: @gb.id )
  #   assert_redirected_to g_game_board_play_url( g_game_board_id: @gb.id )
  #   assert @gb.reload.prof_move?
  # end
  #
  # test "should roll events for investigators on table 2" do
  #   @gb = create( :g_game_board_with_event_ready_for_events_investigators )
  #   @gb.prof_move_end!
  #   @gb.inv_move_end!
  #   @investigator.update_attribute( :event_table, 2 )
  #   get g_game_board_investigators_actions_roll_events_url( g_game_board_id: @gb.id )
  #   assert_redirected_to g_game_board_play_url( g_game_board_id: @gb.id )
  #   assert @gb.reload.prof_move?
  # end

end
