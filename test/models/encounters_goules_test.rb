require 'test_helper'

class EncountersGoulesTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @investigator = @gb.reload.i_investigators.first
    @current_location_code_name = 'oxford'
    @investigator.current_location_code_name = @current_location_code_name
    @investigator.save!
    @goules = create( :goules, g_game_board_id: @gb.id, location_code_name: @investigator.current_location_code_name )
    @inv_san = @investigator.san
  end

  def test_goules_no_weapon_no_sign
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location_code_name, @investigator.current_location_code_name
    assert PMonsterPosition.exists?( @goules.id )
    assert @inv_san-4, @investigator.san
    assert @goules.reload.discovered
  end

  def test_goules_no_weapon_no_sign_investigator_die
    @investigator.update!( san: 2 )
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location_code_name, @investigator.current_location_code_name
    assert PMonsterPosition.exists?( @goules.id )
    assert @inv_san-4, @investigator.san
    assert @investigator.reload.dead?
    assert @goules.reload.discovered
  end

  def test_goules_weapon_bad_shot
    @investigator.update( weapon: true )
    @gb.stubs( :dice ).returns( 1 )
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location_code_name, @investigator.current_location_code_name
    refute PMonsterPosition.exists?( @goules.id )
    assert @inv_san-3, @investigator.san
  end

  def test_goules_weapon_good_shot
    @investigator.update( weapon: true )
    @gb.stubs( :dice ).returns( 6 )
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location_code_name, @investigator.current_location_code_name
    refute PMonsterPosition.exists?( @goules.id )
    assert @inv_san-2, @investigator.san
  end

  def test_goules_weapon_bad_shot_sign
    @investigator.update( weapon: true )
    @investigator.update( sign: true )
    @gb.stubs( :dice ).returns( 1 )
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location_code_name, @investigator.current_location_code_name
    refute PMonsterPosition.exists?( @goules.id )
    assert @inv_san-3, @investigator.san
  end


end