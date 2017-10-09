require 'test_helper'

class InvestigatorIAMovementTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_event_ready_to_move_investigators )
    @investigator = create( :i_investigator, g_game_board_id: @gb.id )
    @professor = @gb.p_professor
  end


  def test_no_encounters_on_psy
    @investigator = @gb.reload.i_investigators.first
    @investigator.update( san: -5 )

    Kernel.stubs( :rand ).returns( 6 )

    @monster = create( :reves, location_code_name: @investigator.current_location_code_name, g_game_board_id: @gb.id )
    @professor.current_location_code_name = :plainfield

    # Investigator should go to psy because he has less than 5 SAN, then he should not encounter dreams.
    @gb.investigators_ia_play( @professor )

    assert_equal 1, @investigator.reload.san
  end

  test 'investigators should all target a spotted professor' do

    imt = GameCore::Ia::ProfPositionFinder.new()
    imt.spot_prof(@gb.turn, @professor.current_location_code_name )
    imt.save( @gb )

    investigator2 = create( :i_investigator, g_game_board_id: @gb.id )
    investigator2.update( weapon: true )
    @investigator.update( weapon: true )

    @gb.investigators_ia_play( @professor )

    assert_equal @professor.current_location_code_name, @investigator.reload.ia_target_destination_code_name
    assert_equal @professor.current_location_code_name, investigator2.reload.ia_target_destination_code_name

  end


  # def setup
  #   @gb = create( :g_game_board_for_inv_movement_tests )
  #   # p CCity.all
  #
  #   # pp @gb.i_investigators
  # end
  #
  # def test_basic_move
  #
  #   # p @gb.i_investigators
  #
  #   @gb.i_investigators.each do |i|
  #     # p i
  #     i.ia_invest_random_move( @gb )
  #   end
  #
  # end

end