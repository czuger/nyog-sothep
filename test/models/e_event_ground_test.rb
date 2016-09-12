require 'test_helper'

class EEventGroundTest < ActiveSupport::TestCase

  def test_all_event_grounds
    gb = create( :g_game_board )
    investigator = gb.i_investigators.first
    professor = gb.p_professor
    EEventLog.start_event_block( gb )
    1.upto(18).each do |i|
      GameCore::EventGroundA.send( "table#{1}_e#{i}", gb, investigator, professor, investigator.current_location )
      investigator.reset unless investigator.normal?
      GameCore::EventGroundB.send( "table#{2}_e#{i}", gb, investigator, professor, investigator.current_location )
      investigator.reset unless investigator.normal?
    end
  end
end
