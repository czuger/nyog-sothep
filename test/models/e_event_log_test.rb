require 'test_helper'

class EEventLogTest < ActiveSupport::TestCase

  test 'Flush event log' do
    @g = create( :g_game_board )
    EEventLog.start_event_block( @g )
    1.upto( 50 ).each do
      EEventLog.log( @g, 'dummy' )
    end
    EEventLog.flush_old_events( @g )
    assert_equal 30, EEventLog.where( g_game_board_id: @g.id ).count
  end

end
