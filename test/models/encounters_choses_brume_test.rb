require 'test_helper'

class EncountersChosesBrumeTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @investigator = @gb.reload.i_investigators.first
    @current_location = @investigator.current_location
    @prof = @gb.p_professor

    @choses_brume = create( :choses_brume, g_game_board_id: @gb.id, location: @investigator.current_location )
    @inv_san = @investigator.san

    IInvestigator.any_instance.stubs(:choose_table ).returns(1)
    IInvestigator.any_instance.stubs(:event_dices ).returns(2)
  end

  def test_choses_brume_miss_turn

    assert @gb.reload.inv_events?

    # Investigtor should be catched in the mists
    @gb.investigators_ia_play( @prof )
    assert_equal @current_location, @investigator.current_location
    refute PMonsterPosition.exists?( @choses_brume.id )

    @gb.prof_movement_done!
    @gb.inv_movement_done!
    assert @gb.inv_events?

    # Investigtor should exit the mists
    @gb.investigators_ia_play( @prof )
    assert_equal @inv_san-1, @investigator.reload.san

    assert @investigator.reload.move?
    assert_equal @current_location, @investigator.current_location
  end

end