require 'test_helper'

class ProfFakePosControllerTest < ActionDispatch::IntegrationTest

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @le_capitaine = create( :le_capitaine, g_game_board_id: @gb.id )
  end

  test "should get new" do
    @gb.ask_prof_for_fake_cities!
    @gb.update( asked_fake_cities_count: 2, asked_fake_cities_investigator: @le_capitaine )

    get new_g_game_board_prof_fake_pos_url( g_game_board_id: @gb.id)
    assert_response :success
    assert_select '#investigator', 'Investigateur actuel : Le capitaine

Selectionnez 2 villes ou le professeur pourrait être. Votre position actuelle sera ajoutée a la liste.'
  end

  test 'should fail because gb state wrong' do
    assert_raise do
      post g_game_board_prof_fake_pos_url( g_game_board_id: @gb.id, cities_ids: [] )
    end
  end

  test 'should fail because cities_ids.count wrong' do
    cities_ids = [ 1, 2, 3, 4 ]
    @gb.ask_prof_for_fake_cities!
    @gb.update( asked_fake_cities_count: 2 )

    assert_raise do
      post g_game_board_prof_fake_pos_url( g_game_board_id: @gb.id, cities_ids: cities_ids )
    end
  end

  test 'should fail because we give the same location as the prof location' do
    cities_ids = [ CCity.first ]
    @gb.ask_prof_for_fake_cities!
    @gb.update( asked_fake_cities_count: 1 )

    @gb.p_professor.update( current_location: CCity.first )

    assert_raise do
      post g_game_board_prof_fake_pos_url( g_game_board_id: @gb.id, cities_ids: cities_ids )
    end
  end

  test 'should set city' do
    cities_ids = [ CCity.second ]
    @gb.ask_prof_for_fake_cities!
    @gb.update( asked_fake_cities_count: 1 )
    @le_capitaine.movement_done!
    @gb.p_professor.update( current_location: CCity.find_by( code_name: :providence ) )
    Kernel.stubs(:rand).returns(1)

    assert_difference 'IInvTargetPosition.count', 2 do
      post g_game_board_prof_fake_pos_url( g_game_board_id: @gb.id, cities_ids: cities_ids )
    end

    assert_redirected_to g_game_board_play_url( @gb )
  end

end