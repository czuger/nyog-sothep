require 'test_helper'

class IInvestigatorTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_cross_border )
    @investigator = @gb.i_investigators.first
    @prof_position_finder = GameCore::Ia::ProfPositionFinder.new
    @imt = GameCore::Ia::InvestigatorMovementTarget.new( @gb, @prof_position_finder )
  end

  test 'go to the psy if low SAN' do
    @investigator.update( san: 1 )

    investigator_movement = GameCore::Ia::InvestigatorMovement.new( @gb, @investigator.reload, @imt )
    investigator_movement.ia_play_movements

    assert @investigator.reload.san > 1
  end

  test 'cross border allowed' do
    Kernel.stubs(:rand).returns(1 )

    @investigator.ia_target_destination_code_name = :providence

    investigator_movement = GameCore::Ia::InvestigatorMovement.new( @gb, @investigator.reload, @imt )
    investigator_movement.ia_play_movements

    assert_not_equal @investigator.last_location_code_name, @investigator.current_location_code_name
  end

  test 'cross border not allowed' do
    Kernel.stubs(:rand).returns(6 )

    @investigator.ia_target_destination_code_name = :providence

    investigator_movement = GameCore::Ia::InvestigatorMovement.new( @gb, @investigator.reload, @imt )
    investigator_movement.ia_play_movements

    assert_equal @investigator.last_location_code_name, @investigator.current_location_code_name
  end

end
