require 'test_helper'

class EncountersGreatPsyTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @investigator = @gb.reload.i_investigators.first
    @current_location = @investigator.current_location

    @choses_brume = create( :choses_brume, g_game_board_id: @gb.id, location: @investigator.current_location )
    @inv_san = @investigator.san

    IInvestigator.any_instance.stubs(:choose_table ).returns(1)
    IInvestigator.any_instance.stubs(:event_dices ).returns(4)
  end

  def test_encounter_great_psy

    GameCore::InvestigatorsActions.new( @gb, @gb.p_professor ).investigators_ia_play

    assert @investigator.reload.in_a_great_psy?

    @gb.prof_movement_done!
    @gb.inv_movement_done!

    GameCore::InvestigatorsActions.new( @gb, @gb.p_professor ).investigators_ia_play

    assert @investigator.reload.move?

  end

end