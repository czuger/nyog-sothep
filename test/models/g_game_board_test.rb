require 'test_helper'

class GGameBoardTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board )
  end

  def test_alive_investigators_should_select_only_living_investigators
    i1 = create( :i_investigator, g_game_board_id: @gb.id )
    i2 = create( :i_investigator, g_game_board_id: @gb.id )
    i2.aasm_die!

    assert_equal 1, @gb.alive_investigators.reload.count
    assert_equal [ i1 ], @gb.alive_investigators.reload
  end

  def test_ready_for_events_investigators_should_not_return_investigator_in_turn_finished_state
    i1 = create( :i_investigator, g_game_board_id: @gb.id )
    i1.update( aasm_state: :turn_finished )

    assert_equal 0, @gb.ready_for_events_investigators.reload.count
    assert_equal [], @gb.ready_for_events_investigators.reload
  end

end
