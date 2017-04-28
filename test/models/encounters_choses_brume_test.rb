require 'test_helper'

class EncountersChosesBrumeTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @investigator = @gb.reload.i_investigators.first
    @current_location = @investigator.current_location

    @choses_brume = create( :choses_brume, g_game_board_id: @gb.id, location: @investigator.current_location )
    @inv_san = @investigator.san

    IInvestigator.any_instance.stubs(:choose_table ).returns(1)
    IInvestigator.any_instance.stubs(:event_dices ).returns(2)
  end

  def test_choses_brume_miss_turn

    assert @gb.reload.inv_events?

    # Investigtor should be catched in the mists
    GameCore::InvestigatorsActions.new( @gb, @gb.p_professor ).investigators_ia_play
    assert_equal @current_location, @investigator.current_location
    refute PMonsterPosition.exists?( @choses_brume.id )

    @gb.prof_movement_done!
    @gb.inv_movement_done!
    assert @gb.inv_events?
    assert_equal @inv_san-2, @investigator.reload.san

    # Investigtor should exit the mists
    GameCore::InvestigatorsActions.new( @gb, @gb.p_professor ).investigators_ia_play

    assert @investigator.reload.move?
    assert_equal @current_location, @investigator.current_location

    assert_equal @inv_san-4, @investigator.reload.san
  end

end