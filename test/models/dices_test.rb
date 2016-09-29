require 'test_helper'

class DicesTest < ActiveSupport::TestCase

  def test_random_dice
    result = GameCore::Dices.d6
    assert result >= 1
    assert result <= 6
  end

  def test_random_double_dice
    result = GameCore::Dices.d6( 2 )
    assert result >= 2
    assert result <= 12
  end

  def test_fixed_double_dice
    Kernel.stubs( :rand ).returns( 3 )
    assert_equal 6, GameCore::Dices.d6( 2 )
  end

  def test_random_big_dice
    result = GameCore::Dices.d42( 3 )
    assert result >= 3
    assert result <= 42*3
  end

end