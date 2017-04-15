require 'test_helper'

class ProfFakePosControllerTest < ActionDispatch::IntegrationTest

  def setup
    @gb = create( :g_game_board )
  end

  test "should get new" do
    get new_g_game_board_prof_fake_pos_url( g_game_board_id: @gb.id, nb_cities: 3 )
    assert_response :success
    assert_select '#investigator', 'Selectionnez 3 villes ou le professeur pourrait être. Votre position actuelle sera ajoutée a la liste.'
  end

  test "should get create" do
    cities_ids = [ 1, 2, 3, 4 ]
    post g_game_board_prof_fake_pos_url( g_game_board_id: @gb.id, cities_ids: cities_ids )
    assert_redirected_to g_game_board_play_url( id: @gb.id )
  end

end
