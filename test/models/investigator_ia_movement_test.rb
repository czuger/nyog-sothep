require 'test_helper'

class InvestigatorIAMovementTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_for_inv_movement_tests )
    # p CCity.all

    # pp @gb.i_investigators
  end

  def test_basic_move

    # p @gb.i_investigators

    @gb.i_investigators.each do |i|
      # p i
      i.ia_invest_random_move( @gb )
    end

  end

end