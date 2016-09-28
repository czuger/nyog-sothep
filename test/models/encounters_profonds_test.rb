require 'test_helper'

class EncountersProfondsTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @investigator = @gb.i_investigators.first
    @current_location = @investigator.current_location
    @profonds = create( :profonds, g_game_board_id: @gb.id, location: @investigator.current_location )
    EEventLog.start_event_block( @gb )
    @inv_san = @investigator.san
  end

  def test_profonds_no_sign
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location, @investigator.current_location
    assert PMonsterPosition.exists?( @profonds.id )
    assert @inv_san-3, @investigator.san
    assert @profonds.reload.discovered
    assert_equal @current_location, @profonds.location
  end

  def test_profonds_no_sign_inv_die
    @investigator.update!( san: 2 )
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location, @investigator.current_location
    assert PMonsterPosition.exists?( @profonds.id )
    assert @inv_san-3, @investigator.san
    assert @profonds.reload.discovered
    assert_equal @current_location, @profonds.location
    assert @investigator.reload.dead
  end

  def test_profonds_with_sign
    @investigator.update( sign: true )
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location, @investigator.current_location
    refute PMonsterPosition.exists?( @profonds.id )
    assert @inv_san, @investigator.san
    refute @investigator.sign
  end

end