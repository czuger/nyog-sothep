require 'test_helper'

class InvestigatorsActionsControllerTest < ActionDispatch::IntegrationTest

  def setup
    @gb = create( :g_game_board )
    @investigator = @gb.i_investigators.first
    @dest = @investigator.current_location.destinations.first
    @gb.start_turn!
    @gb.prof_move_end!
    @gb.prof_attack_end!
  end

  test "should go psy" do
    get go_psy_g_game_board_investigators_action_url( g_game_board_id: @gb.id, id: @investigator.id  )
    assert_redirected_to investigators_map_show_url
  end

  test "investigator should move" do
    get move_g_game_board_investigators_action_url( g_game_board_id: @gb.id, id: @investigator.id, zone_id: @dest.id, zone_class: @dest.class )
    assert_redirected_to investigators_map_show_url
    assert @gb.inv_move?
  end

  test "investigator should through a border and fail" do
    border_cross_road = create( :inv_cross_border_road )
    @investigator.current_location = border_cross_road.src_city
    @investigator.save!
    @dest = border_cross_road.dest_city
    InvestigatorsActionsController.any_instance.stubs(:d6).returns(6) # Stubs the dice method in order it fails always

    get move_g_game_board_investigators_action_url( g_game_board_id: @gb.id, id: @investigator.id, zone_id: @dest.id, zone_class: @dest.class )
    assert_redirected_to investigators_map_show_url
    assert_equal border_cross_road.src_city, @investigator.reload.current_location
    assert @gb.inv_move?
  end

  test 'investigators should be ready even if no event happens (water movement)' do
    @gb = create( :g_game_board_with_event_ready_investigators )
    @gb.start_turn!
    @gb.prof_move_end!
    @gb.prof_attack_end!
    @investigator = @gb.i_investigators.where( aasm_state: :ready ).first
    wwac = create( :w_water_area_connection )
    @dest = wwac.dest_w_water_area
    @investigator.current_location = wwac.src_w_water_area
    get move_g_game_board_investigators_action_url( g_game_board_id: @gb.id, id: @investigator.id, zone_id: @dest.id, zone_class: @dest.class )
    assert_redirected_to investigators_map_show_url
    assert_equal 0, @gb.i_investigators.where( aasm_state: :move_done ).count
    assert @gb.i_investigators.where( aasm_state: [ :ready, :known_psy_help, :delayed, :replay ] ).order( :id ).first
  end

  test "should roll events for investigators on table 1" do
    @gb = create( :g_game_board_with_event_ready_investigators )
    @gb.start_turn!
    @gb.prof_move_end!
    @gb.prof_attack_end!
    @investigator = @gb.i_investigators.where( aasm_state: :ready ).first
    @dest = @investigator.current_location.destinations.first
    get move_g_game_board_investigators_action_url( g_game_board_id: @gb.id, id: @investigator.id, zone_id: @dest.id, zone_class: @dest.class )
    assert_redirected_to investigators_map_show_url
    assert_equal 0, @gb.i_investigators.where( aasm_state: :move_done ).count
    assert @gb.i_investigators.where( aasm_state: [ :ready, :known_psy_help, :delayed, :replay ] ).order( :id ).first
    assert @gb.i_investigators.where( aasm_state: [ :ready, :known_psy_help, :delayed, :replay ] ).order( :id ).first
    assert @gb.reload.start?
  end

  test "should roll events for investigators on table 2" do
    @gb = create( :g_game_board_with_event_ready_investigators )
    @gb.start_turn!
    @gb.prof_move_end!
    @gb.prof_attack_end!
    @investigator = @gb.i_investigators.where( aasm_state: :ready ).first
    @investigator.update_attribute( :event_table, 2 )
    @dest = @investigator.current_location.destinations.first
    get move_g_game_board_investigators_action_url( g_game_board_id: @gb.id, id: @investigator.id, zone_id: @dest.id, zone_class: @dest.class )
    assert_redirected_to investigators_map_show_url
    assert_equal 0, @gb.i_investigators.where( aasm_state: :move_done ).count
    assert @gb.i_investigators.where( aasm_state: [ :ready, :known_psy_help, :delayed, :replay ] ).order( :id ).first
    assert @gb.reload.start?
  end

end
