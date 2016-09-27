require 'test_helper'

class FullTurnTest < ActionDispatch::IntegrationTest

  test 'A full turn' do

    @gb = create( :g_game_board_with_event_ready_to_move_investigators )
    @gb.i_investigators.last.update( { event_table: 2 } )
    @avoidance_list = [ [ 17, 1 ], [ 14, 1 ], [ 10, 2 ], [ 11, 2 ], [ 4, 2 ], [ 9, 1 ] ]

    get '/'
    assert_response :success

    1.upto(18).each do |dices_result|
      # puts '*'*100
      # puts "Dice result : #{dices_result}"

      1.upto(2).each do |i|
        # puts "Turn : #{i}"
        @prof = @gb.p_professor
        @prof_dest = @prof.current_location.destinations.first

        # We call it because normally use should call it
        get map_show_url
        get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: @prof_dest.id, zone_class: @prof_dest.class )
        assert_redirected_to map_show_url

        follow_redirect!
        assert_response :success

        1.upto(4).each do

          inv = @gb.next_moving_investigator

          # puts [ dices_result, inv.event_table ].inspect
          if @avoidance_list.include?( [ dices_result, inv.event_table ] )
            dices_result = 1
          end

          InvestigatorsActionsController.any_instance.stubs( :event_dices ).returns( dices_result )
          MapController.any_instance.stubs( :event_dices ).returns( dices_result )

          if inv&.inv_move?
            # puts "Investigator : #{inv.code_name}, Event table : #{inv.event_table}"
            road = create( :inv_road )
            inv.current_location = road.src_city
            inv.save!
            inv_dest = inv.current_location.destinations.first

            # We call it because normally use should call it
            get map_show_url
            get move_g_game_board_investigators_action_url( g_game_board_id: @gb.id, id: inv.id, zone_id: inv_dest.id, zone_class: inv_dest.class )
            assert_redirected_to map_show_url

            follow_redirect!
            assert_response :success
          end

        end
      end
    end
  end
end
