require 'test_helper'

class EncountersFanatiquesTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @investigator = @gb.reload.i_investigators.first
    @last_location_code_name = @investigator.last_location_code_name
    current_location_code_name = :oxford
    @investigator.current_location_code_name = current_location_code_name
    @investigator.save!
    @fanatiques = create( :fanatiques, g_game_board_id: @gb.id, location_code_name: @investigator.current_location_code_name )
    @inv_san = @investigator.san
  end

  def test_fanatiques_no_weapons
    @gb.resolve_encounter( @investigator )
    assert_equal @last_location_code_name, @investigator.current_location_code_name
    assert PMonsterPosition.exists?( @fanatiques.id )
    assert @inv_san, @investigator.san
    assert @fanatiques.reload.discovered
  end

  def test_fanatiques_weapons_failure
    @investigator.update_attribute( :weapon, true )
    Kernel.stubs( :rand ).returns( 1 )
    @gb.resolve_encounter( @investigator )
    assert_equal @last_location_code_name, @investigator.current_location_code_name
    assert PMonsterPosition.exists?( @fanatiques.id )
    assert @inv_san, @investigator.san
    assert @fanatiques.reload.discovered
  end

  def test_fanatiques_weapons_success
    @investigator.update_attribute( :weapon, true )
    Kernel.stubs( :rand ).returns( 6 )
    @gb.resolve_encounter( @investigator )
    # refute_equal @last_location_code_name, @investigator.current_location_code_name
    refute PMonsterPosition.exists?( @fanatiques.id )
    assert @inv_san - 2, @investigator.san
  end

  def test_fanatiques_weapons_critical
    @investigator.update_attribute( :weapon, true )
    @investigator.update_attribute( :medaillon, true )
    Kernel.stubs( :rand ).returns( 6 )
    @gb.resolve_encounter( @investigator )
    # refute_equal @last_location_code_name, @investigator.current_location_code_name
    refute PMonsterPosition.exists?( @fanatiques.id )
    assert @inv_san, @investigator.san
  end

end