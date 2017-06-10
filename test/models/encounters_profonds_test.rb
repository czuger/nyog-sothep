require 'test_helper'

class EncountersProfondsTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @investigator = @gb.reload.i_investigators.first
    @current_location_code_name = @investigator.current_location_code_name
    @profonds = create( :profonds, g_game_board_id: @gb.id, location_code_name: @investigator.current_location_code_name )
    @inv_san = @investigator.san
  end

  def test_profonds_no_sign
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location_code_name, @investigator.current_location_code_name
    assert PMonsterPosition.exists?( @profonds.id )
    assert @inv_san-3, @investigator.san
    assert @profonds.reload.discovered
    assert_equal @current_location_code_name, @profonds.location_code_name
  end

  def test_profonds_no_sign_inv_die
    @investigator.update!( san: 2 )
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location_code_name, @investigator.current_location_code_name
    assert PMonsterPosition.exists?( @profonds.id )
    assert @inv_san-3, @investigator.san
    assert @profonds.reload.discovered
    assert_equal @current_location_code_name, @profonds.location_code_name
    assert @investigator.reload.dead?
  end

  def test_profonds_with_sign
    @investigator.update( sign: true )
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location_code_name, @investigator.current_location_code_name
    refute PMonsterPosition.exists?( @profonds.id )
    assert @inv_san, @investigator.san
    refute @investigator.sign
  end

end