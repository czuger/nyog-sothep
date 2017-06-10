require 'test_helper'

class EncountersHabitantsTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @investigator = @gb.reload.i_investigators.first
    @last_location_code_name = @investigator.last_location_code_name
    @current_location_code_name = :oxford
    @investigator.current_location_code_name = @current_location_code_name
    @investigator.save!
    @habitants = create( :habitants, g_game_board_id: @gb.id, location_code_name: @investigator.current_location_code_name )
  end

  def test_habitants_no_sign
    @investigator.update( medaillon: false )
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location_code_name, @investigator.reload.current_location_code_name.to_sym
    assert PMonsterPosition.exists?( @habitants.id )
  end

  def test_habitants_sign
    @investigator.update( medaillon: true )
    @gb.resolve_encounter( @investigator )
    assert_equal @last_location_code_name, @investigator.reload.current_location_code_name
    assert_equal @current_location_code_name, @investigator.reload.forbidden_city_code_name.to_sym
    refute PMonsterPosition.exists?( @habitants.id )
  end

end