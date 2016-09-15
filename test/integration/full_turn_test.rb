require 'test_helper'

class FullTurnTest < ActionDispatch::IntegrationTest

  test 'A full turn' do

    @gb = create( :g_game_board_with_event_ready_to_move_investigators )
    @gb.i_investigators.last.update( { event_table: 2 } )

    get '/'
    assert_response :success

    1.upto(18).each do |dices_result|
      InvestigatorsActionsController.any_instance.stubs( :event_dices ).returns( dices_result )
      1.upto(2).each do |i|
        puts "Turn : #{i}"
        @prof = @gb.p_professor
        @prof_dest = @prof.current_location.destinations.first
        get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: @prof_dest.id, zone_class: @prof_dest.class )
        assert_redirected_to map_show_url

        follow_redirect!
        assert_response :success

        @gb.i_investigators.each do |inv|
          # puts inv.inspect
          road = create( :inv_road )
          inv.current_location = road.src_city
          inv.save!
          inv_dest = inv.current_location.destinations.first

          if inv.reload.ready_to_move?
            get move_g_game_board_investigators_action_url( g_game_board_id: @gb.id, id: inv.id, zone_id: inv_dest.id, zone_class: inv_dest.class )

            assert_redirected_to map_show_url

            follow_redirect!
            assert_response :success
          end

          # get special_event_g_game_board_investigators_action_url( g_game_board_id: @gb.id, id: inv.id )
          # else

        end

        get g_game_board_investigators_actions_roll_events_url( g_game_board_id: @gb.id )
        assert_redirected_to map_show_url

        follow_redirect!
        assert_response :success
      end
    end
  end
end
