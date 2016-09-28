require 'test_helper'

class ProfessorActionsControllerTest < ActionDispatch::IntegrationTest

  def setup
    @gb = create( :g_game_board )
    @professor = @gb.p_professor
    @dest = @professor.current_location.destinations.first
  end

  test 'professor should move' do
    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: @dest.id, zone_class: @dest.class )
    assert_redirected_to map_show_url
  end

  test 'professor should breed' do
    monster = create( :p_monster, g_game_board_id: @gb.id )
    @gb.update( aasm_state: 'prof_breed' )
    get monster_breed_g_game_board_professor_actions_url( g_game_board_id: @gb.id, monster_id: monster.id )
    assert_redirected_to map_show_url
  end

end
