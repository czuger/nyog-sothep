require 'test_helper'

class ProfessorActionsControllerTest < ActionDispatch::IntegrationTest

  def setup
    @gb = create( :g_game_board_ready_for_fight )
    @professor = @gb.p_professor
    @dest = @professor.current_location.destinations.first
    @investigator = create( :i_investigator, g_game_board_id: @gb.id )
    @investigator.update( weapon: true )
  end

  # TODO : replace this tests as unit tests
  # test 'professor should fight and loose hard' do
  #   Kernel.stubs(:rand).returns(6)
  #   get attack_g_game_board_professor_actions_url( g_game_board_id: @gb.id, investigator_id: @investigator.id )
  #   assert_redirected_to g_game_board_play_url(  attacking_investigator_id: @investigator.id )
  # end
  #
  # test 'professor should fight and loose' do
  #   Kernel.stubs(:rand).returns(3)
  #   get attack_g_game_board_professor_actions_url( g_game_board_id: @gb.id, investigator_id: @investigator.id )
  #   assert_redirected_to g_game_board_play_url( attacking_investigator_id: @investigator.id )
  # end
  #
  # test 'professor should fight and nobody wins' do
  #   Kernel.stubs(:rand).returns(1)
  #   get attack_g_game_board_professor_actions_url( g_game_board_id: @gb.id, investigator_id: @investigator.id )
  #   assert_redirected_to g_game_board_play_url( attacking_investigator_id: @investigator.id )
  # end
  #
  # test 'professor should fight and investigator is protected by sign' do
  #   @investigator.update( weapon: false, sign: true )
  #   get attack_g_game_board_professor_actions_url( g_game_board_id: @gb.id, investigator_id: @investigator.id )
  #   assert_redirected_to g_game_board_play_url( attacking_investigator_id: @investigator.id )
  # end
  #
  # test 'professor should fight and investigator is crushed' do
  #   @investigator.update( weapon: false, sign: false )
  #   get attack_g_game_board_professor_actions_url( g_game_board_id: @gb.id, investigator_id: @investigator.id )
  #   assert_redirected_to g_game_board_play_url( attacking_investigator_id: @investigator.id )
  # end

  test 'professor should move' do
    @gb.update( aasm_state: 'prof_move' )
    Kernel.stubs(:rand).returns(1)

    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: @dest.id, zone_class: @dest.class )
    assert_redirected_to g_game_board_play_url
  end

  test 'professor should move then be asked for fake cities - test with only one investigators' do
    @gb.update( aasm_state: 'prof_move' )

    # Expects are stacked and played in inverse order
    IInvestigator.any_instance.stubs(:choose_table ).returns(1)
    IInvestigator.any_instance.stubs(:event_dices ).returns(5)
    # IInvestigator.expects(:event_dices).returns(5)

    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: @dest.id, zone_class: @dest.class )
    assert_redirected_to new_g_game_board_prof_fake_pos_url
  end

  test 'professor should move then be asked for fake cities - test with all investigators' do
    @gb.update( aasm_state: 'prof_move' )

    # Expects are stacked and played in inverse order
    IInvestigator.any_instance.stubs(:choose_table ).returns(1)
    IInvestigator.any_instance.stubs(:event_dices ).returns(5)
    # IInvestigator.expects(:event_dices).returns(5)

    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: @dest.id, zone_class: @dest.class )
    assert_redirected_to new_g_game_board_prof_fake_pos_url
  end

  test 'professor should breed' do
    monster = create( :p_monster, g_game_board_id: @gb.id )
    @gb.update( aasm_state: 'prof_breed' )
    get monster_breed_g_game_board_professor_actions_url( g_game_board_id: @gb.id, monster_id: monster.id )
    assert_redirected_to g_game_board_play_url
  end

  test 'professor should move, then give 2 fake positions, then move again' do
    @gb.update( aasm_state: 'prof_move' )
    IInvestigator.any_instance.stubs(:choose_table ).returns(1)
    IInvestigator.any_instance.stubs(:event_dices ).returns(5)

    src = CCity.find_by_code_name( :oxford )
    dest = CCity.find_by_code_name( :plainfield )
    third_city = CCity.find_by_code_name( :providence )

    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: dest.id, zone_class: dest.class )

    IInvestigator.any_instance.stubs(:choose_table ).returns(1)
    IInvestigator.any_instance.stubs(:event_dices ).returns(1)

    cities_ids = [ src, third_city ]
    post g_game_board_prof_fake_pos_url( g_game_board_id: @gb.id, cities_ids: cities_ids )

    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: src.id, zone_class: src.class )

    assert_redirected_to g_game_board_play_url
  end

  test 'professor should move 5 times' do
    @gb.update( aasm_state: 'prof_move' )
    Kernel.stubs(:rand).returns(1)

    1.upto( 6 ).each do
      create( :i_investigator, g_game_board_id: @gb.id, current_location: @investigator.current_location,
                             last_location: @investigator.last_location )
    end

    src = CCity.find_by_code_name( :oxford )
    dest = CCity.find_by_code_name( :plainfield )

    1.upto(10) do

      get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: dest.id, zone_class: dest.class )
      tmp = dest
      dest = src
      src = tmp
    end

    assert_redirected_to g_game_board_play_url
  end

end
