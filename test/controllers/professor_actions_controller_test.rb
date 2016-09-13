require 'test_helper'

class ProfessorActionsControllerTest < ActionDispatch::IntegrationTest

  def setup
    @gb = create( :g_game_board )
    @gb.start_turn!
    @professor = @gb.p_professor
    @dest = @professor.current_location.destinations.first
  end

  test 'professor should move' do
    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, id: @professor.id, zone_id: @dest.id, zone_class: @dest.class )
    assert_redirected_to professor_map_show_url
  end

end
