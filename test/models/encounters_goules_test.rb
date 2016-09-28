require 'test_helper'

class EncountersGoulesTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @investigator = @gb.i_investigators.first
    current_location = create( :inv_dest_city )
    @investigator.current_location = current_location
    @investigator.save!
    @current_location = @investigator.current_location
    @goules = create( :goules, g_game_board_id: @gb.id, location: @investigator.current_location )
    EEventLog.start_event_block( @gb )
    @inv_san = @investigator.san
  end

  def test_goules_no_weapon_no_sign
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location, @investigator.current_location
    assert PMonsterPosition.exists?( @goules.id )
    assert @inv_san-4, @investigator.san
    assert @goules.reload.discovered
  end

  def test_goules_no_weapon_no_sign_investigator_die
    @investigator.update!( san: 2 )
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location, @investigator.current_location
    assert PMonsterPosition.exists?( @goules.id )
    assert @inv_san-4, @investigator.san
    assert @investigator.reload.dead
    assert @goules.reload.discovered
  end

  def test_goules_weapon_bad_shot
    @investigator.update( weapon: true )
    @gb.stubs( :dice ).returns( 1 )
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location, @investigator.current_location
    refute PMonsterPosition.exists?( @goules.id )
    assert @inv_san-3, @investigator.san
  end

  def test_goules_weapon_good_shot
    @investigator.update( weapon: true )
    @gb.stubs( :dice ).returns( 6 )
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location, @investigator.current_location
    refute PMonsterPosition.exists?( @goules.id )
    assert @inv_san-2, @investigator.san
  end

  def test_goules_weapon_bad_shot_sign
    @investigator.update( weapon: true )
    @investigator.update( sign: true )
    @gb.stubs( :dice ).returns( 1 )
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location, @investigator.current_location
    refute PMonsterPosition.exists?( @goules.id )
    assert @inv_san-3, @investigator.san
  end


end