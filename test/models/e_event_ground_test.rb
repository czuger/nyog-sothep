require 'test_helper'

class EEventGroundTest < ActiveSupport::TestCase

  def test_all_event_grounds
    gb = create( :g_game_board_with_event_ready_for_events_investigators )
    investigator = gb.i_investigators.first
    professor = gb.p_professor
    EEventLog.start_event_block( gb )
    1.upto(18).each do |i|

      investigator.update_attribute( :aasm_state, :move_phase_done )
      GameCore::EventGroundA.send( "table#{1}_e#{i}", gb, investigator, professor )

      investigator.update_attribute( :aasm_state, :move_phase_done )
      GameCore::EventGroundB.send( "table#{2}_e#{i}", gb, investigator, professor )
    end
  end
end
