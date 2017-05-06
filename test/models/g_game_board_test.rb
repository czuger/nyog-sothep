require 'test_helper'

class GGameBoardTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board )
  end

  def test_alive_investigators_should_select_only_living_investigators
    i1 = create( :i_investigator, g_game_board_id: @gb.id )
    i2 = create( :i_investigator, g_game_board_id: @gb.id )
    i2.die!

    assert_equal 1, @gb.alive_investigators.reload.count
    assert_equal [ i1 ], @gb.alive_investigators.reload
  end

  # def test_game_board_end_turn
  #   @gb = create( :g_game_board_with_event_ready_for_events_investigators )
  #   @gb.prof_move!
  #   assert @gb.prof_move?
  # end
  #
  # def test_professor_pick_start_monster
  #   @gb = create( :g_game_board_with_event_ready_for_events_investigators )
  #   @gb.prof_move!
  #   EEventLog.start_event_block( @gb )
  #   1.upto(20).each do
  #     create( :m_monster, g_game_board_id: @gb.id )
  #   end
  #   assert_difference '@gb.p_monsters.count', 4 do
  #     @gb.professor_pick_start_monsters
  #   end
  # end

end
