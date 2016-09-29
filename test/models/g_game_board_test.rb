require 'test_helper'

class GGameBoardTest < ActiveSupport::TestCase

  def test_game_board_end_turn
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @gb.inv_event_end!
    assert @gb.prof_move?
  end

  def test_professor_pick_start_monster
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @gb.inv_event_end!
    EEventLog.start_event_block( @gb )
    1.upto(20).each do
      create( :m_monster, g_game_board_id: @gb.id )
    end
    assert_difference '@gb.p_monsters.count', 4 do
      @gb.professor_pick_start_monsters
    end
  end

end
