require 'test_helper'

class EncountersHabitantsTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @investigator = @gb.reload.i_investigators.first
    @last_location = @investigator.last_location
    @current_location = CCity.find_by( code_name: :oxford ) || create( :oxford )
    @investigator.current_location = @current_location
    @investigator.save!
    @habitants = create( :habitants, g_game_board_id: @gb.id, location: @investigator.current_location )
  end

  def test_habitants_no_sign
    @investigator.update( medaillon: false )
    @gb.resolve_encounter( @investigator )
    assert_equal @current_location, @investigator.current_location
    assert PMonsterPosition.exists?( @habitants.id )
  end

  def test_habitants_sign
    @investigator.update( medaillon: true )
    @gb.resolve_encounter( @investigator )
    assert_equal @last_location, @investigator.current_location
    assert_equal @current_location, @investigator.forbidden_city
    refute PMonsterPosition.exists?( @habitants.id )
  end

end