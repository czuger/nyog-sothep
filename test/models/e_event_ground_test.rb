require 'test_helper'

class EEventGroundTest < ActiveSupport::TestCase
  #
  # def setup
  #   @gb = create( :g_game_board_with_event_ready_for_events_investigators )
  #   @investigator = @gb.reload.i_investigators.first
  #   @professor = @gb.p_professor
  # end
  #
  # def test_events_on_table_2
  #   @investigator.update( event_table: 2 )
  #   Kernel.stubs(:rand).returns(3)
  #   GameCore::Events.roll_event( @gb, @investigator, @professor )
  #   assert @investigator.reload.spell
  # end
  #
  # def test_no_event_on_water
  #   water_area = create( :w_water_area )
  #   @investigator.current_location = water_area
  #   @investigator.save!
  #   Kernel.stubs(:rand).returns(2)
  #   GameCore::Events.roll_event( @gb, @investigator, @professor )
  #   refute @investigator.reload.weapon
  # end
  #
  # def test_all_event_grounds
  #   1.upto(18).each do |i|
  #     @investigator.update_attribute( :aasm_state, :roll_event )
  #     GameCore::EventGroundA.send( "table#{1}_e#{i}", @gb, @investigator, @professor )
  #
  #     @investigator.update_attribute( :aasm_state, :roll_event )
  #     GameCore::EventGroundB.send( "table#{2}_e#{i}", @gb, @investigator, @professor )
  #   end
  #   # No assertion, this is just a pass through test
  # end

end
