require 'test_helper'

class EncountersGreatPsyTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @investigator = @gb.reload.i_investigators.first
    @current_location = @investigator.current_location
    @prof = @gb.p_professor

    @inv_san = @investigator.san

    IInvestigator.any_instance.stubs(:choose_table ).returns(1)
    IInvestigator.any_instance.stubs(:event_dices ).returns(4)
  end

  def test_encounter_great_psy

    # p @gb.aasm_state

    @gb.investigators_ia_play( @prof )

    # p @investigator.aasm_state

    assert @investigator.reload.at_a_great_psy?

    @gb.prof_movement_done!
    @gb.inv_movement_done!

    @gb.investigators_ia_play( @prof )

    assert @investigator.reload.move?
    assert_equal @inv_san + 5, @investigator.reload.san

  end

end