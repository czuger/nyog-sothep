require 'test_helper'

class EncountersFanatiquesTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @investigator = @gb.reload.i_investigators.first
    @last_location = @investigator.last_location
    current_location = CCity.find_by( code_name: :oxford ) || create( :oxford )
    @investigator.current_location = current_location
    @investigator.save!
    @fanatiques = create( :fanatiques, g_game_board_id: @gb.id, location: @investigator.current_location )
    @inv_san = @investigator.san
  end

  def test_fanatiques_no_weapons
    @gb.resolve_encounter( @investigator )
    assert_equal @last_location, @investigator.current_location
    assert PMonsterPosition.exists?( @fanatiques.id )
    assert @inv_san, @investigator.san
    assert @fanatiques.reload.discovered
  end

  def test_fanatiques_weapons_failure
    @investigator.update_attribute( :weapon, true )
    Kernel.stubs( :rand ).returns( 1 )
    @gb.resolve_encounter( @investigator )
    assert_equal @last_location, @investigator.current_location
    assert PMonsterPosition.exists?( @fanatiques.id )
    assert @inv_san, @investigator.san
    assert @fanatiques.reload.discovered
  end

  def test_fanatiques_weapons_success
    @investigator.update_attribute( :weapon, true )
    Kernel.stubs( :rand ).returns( 6 )
    @gb.resolve_encounter( @investigator )
    # refute_equal @last_location, @investigator.current_location
    refute PMonsterPosition.exists?( @fanatiques.id )
    assert @inv_san - 2, @investigator.san
  end

  def test_fanatiques_weapons_critical
    @investigator.update_attribute( :weapon, true )
    @investigator.update_attribute( :medaillon, true )
    Kernel.stubs( :rand ).returns( 6 )
    @gb.resolve_encounter( @investigator )
    # refute_equal @last_location, @investigator.current_location
    refute PMonsterPosition.exists?( @fanatiques.id )
    assert @inv_san, @investigator.san
  end

end