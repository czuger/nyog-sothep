require 'test_helper'

class ProfessorActionsControllerTest < ActionDispatch::IntegrationTest

  def setup
    @gb = create( :g_game_board_ready_for_fight )
    @professor = @gb.p_professor

    prof_location = GameCore::Map::Location.get_location( @professor.current_location_code_name )
    @dest = prof_location.destinations.first

    @investigator = create( :i_investigator, g_game_board_id: @gb.id )
    @investigator.update( weapon: true )
  end

  test 'nyog sothep repelling is successfull' do

    @gb.destroy
    @gb = create( :g_game_board, nyog_sothep_current_location_code_name: :oxford, nyog_sothep_repelling_city_code_name: :oxford )
    @i1 = create( :repelling_investigator, g_game_board: @gb )
    @i2 = create( :repelling_investigator, g_game_board: @gb )
    @i3 = create( :repelling_investigator, g_game_board: @gb, nyog_sothep_already_seen: true )
    @prof = @gb.p_professor
    @prof.update( current_location_code_name: :plainfield )

    GGameBoard.any_instance.stubs( :repelling_roll ).returns( -2 )

    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: :oxford )
    assert_redirected_to g_game_board_play_url
  end

  test 'professor should move' do
    @gb.update( aasm_state: 'prof_move' )
    Kernel.stubs(:rand).returns(1)

    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: @dest.code_name )
    assert_redirected_to g_game_board_play_url
  end

  test 'professor should move then be asked for fake cities - test with only one investigators' do
    @gb.update( aasm_state: 'prof_move' )

    # Expects are stacked and played in inverse order
    IInvestigator.any_instance.stubs(:choose_table ).returns(1)
    IInvestigator.any_instance.stubs(:event_dices ).returns(5)
    # IInvestigator.expects(:event_dices).returns(5)

    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: @dest.code_name )
    assert_redirected_to g_game_board_play_url
  end

  test 'professor should move then be asked for fake cities - test with all investigators' do
    @gb.update( aasm_state: 'prof_move' )

    # Expects are stacked and played in inverse order
    IInvestigator.any_instance.stubs(:choose_table ).returns(1)
    IInvestigator.any_instance.stubs(:event_dices ).returns(5)
    # IInvestigator.expects(:event_dices).returns(5)

    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: @dest.code_name )
    assert_redirected_to g_game_board_play_url
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

    src = :oxford
    dest = :plainfield
    third_city = :providence

    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: dest )

    IInvestigator.any_instance.stubs(:choose_table ).returns(1)
    IInvestigator.any_instance.stubs(:event_dices ).returns(1)

    cities_ids = [ src, third_city ]
    post g_game_board_prof_fake_pos_url( g_game_board_id: @gb.id, cities_ids: cities_ids )

    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: src )

    assert_redirected_to g_game_board_play_url
  end

  test 'professor should move n times' do
    @gb.update( aasm_state: 'prof_move' )
    Kernel.stubs(:rand).returns(1)

    1.upto( 6 ).each do
      create( :i_investigator, g_game_board_id: @gb.id, current_location_code_name: @investigator.current_location_code_name,
                             last_location_code_name: @investigator.last_location_code_name )
    end

    src = :oxford
    dest = :plainfield

    tests_amount = 3
    1.upto(tests_amount) do

      get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: dest )
      tmp = dest
      dest = src
      src = tmp
    end

    assert_redirected_to g_game_board_play_url
  end

  test 'spotted professor should move, all investigators should target him if spotted' do
    @gb.update( aasm_state: 'prof_move' )
    Kernel.stubs(:rand).returns(1)

    imt = GameCore::Ia::ProfPositionFinder.new()
    imt.spot_prof(@gb.turn, @professor.current_location_code_name )
    imt.save( @gb )

    create( :weapon_ready_investigator, g_game_board: @gb )
    create( :weapon_ready_investigator, g_game_board: @gb )

    get move_g_game_board_professor_actions_url( g_game_board_id: @gb.id, zone_id: @dest.code_name )
    assert_redirected_to g_game_board_play_url

    @gb.i_investigators.each_with_index do |inv, idx|
      assert_equal @professor.current_location_code_name, inv.reload.ia_target_destination_code_name
    end
  end

end
