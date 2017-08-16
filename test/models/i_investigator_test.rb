require 'test_helper'

class IInvestigatorTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_cross_border )
    @investigator = @gb.i_investigators.first
    @imt = GameCore::Ia::InvestigatorMovementTarget.new( self )
  end

  test 'go to the psy if low SAN' do
    @investigator.update( san: 1 )
    @investigator.ia_play_movements( @gb, @imt )
    assert @investigator.reload.san > 1
  end

  test 'cross border allowed' do
    Kernel.stubs(:rand).returns(1 )

    @investigator.ia_target_destination_code_name = :providence

    @investigator.reload.ia_play_movements( @gb, @imt )

    assert_not_equal @investigator.last_location_code_name, @investigator.current_location_code_name
  end

  test 'cross border not allowed' do
    Kernel.stubs(:rand).returns(6 )

    @investigator.ia_target_destination_code_name = :providence

    @investigator.reload.ia_play_movements( @gb, @imt )

    assert_equal @investigator.last_location_code_name, @investigator.current_location_code_name
  end

end
