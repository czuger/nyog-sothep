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

    @monster = create( :reves, location: @investigator.current_location, g_game_board_id: @gb.id )
    @professor.current_location = CCity.find_by( code_name: :plainfield )

    # Investigator should go to psy because he has less than 5 SAN, then he should not encounter dreams.
    @gb.investigators_ia_play( @professor )

    assert_equal 1, @investigator.reload.san
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