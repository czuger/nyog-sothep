require 'test_helper'

class GGameBoardsControllerTest < ActionDispatch::IntegrationTest
  setup do
    create( :c_city, code_name: :nantucket )
    create( :w_water_area, code_name: :nantucket_sound )
    @g_game_board = create( :g_game_board_with_event_ready_to_move_investigators )
  end

  test "should get index" do
    get g_game_boards_url
    assert_response :success
  end

  test "should get new" do
    get new_g_game_board_url
    assert_response :success
  end

  test "should create g_game_board" do
    assert_difference('GGameBoard.count') do
      post g_game_boards_url, params: { g_game_board: { ia_side: @g_game_board.ia_side, players_count: @g_game_board.players_count } }
    end

    assert_redirected_to g_game_board_url(GGameBoard.last)
  end

  test "should destroy g_game_board" do
    assert_difference('GGameBoard.count', -1) do
      delete g_game_board_url(@g_game_board)
    end

    assert_redirected_to g_game_boards_url
  end
end
