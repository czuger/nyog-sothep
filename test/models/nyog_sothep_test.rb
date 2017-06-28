require 'test_helper'

class NyogSothepTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board, nyog_sothep_current_location_code_name: :oxford, nyog_sothep_repelling_city_code_name: :oxford )
    @san = 30
    @i1 = create( :repelling_investigator, g_game_board: @gb )
    @i2 = create( :repelling_investigator, g_game_board: @gb )
    @i3 = create( :repelling_investigator, g_game_board: @gb, nyog_sothep_already_seen: true )
  end

  test 'loose_game' do
    @gb.stubs( :repelling_roll ).returns( -2 )
    @gb.nyog_sothep_repelled_test
    assert @gb.game_lost?
  end

  def test_worst_repelling_test
    @gb.stubs( :repelling_roll ).returns( 3 )
    @gb.nyog_sothep_repelled_test
    assert_equal @san - 8, @i1.reload.san
    assert_equal @san - 8, @i2.reload.san
    assert_equal @san - 6, @i3.reload.san
  end

  def test_bad_repelling_test
    @gb.stubs( :repelling_roll ).returns( 2 )
    @gb.nyog_sothep_repelled_test
    assert_equal @san - 6, @i1.reload.san
    assert_equal @san - 6, @i2.reload.san
    assert_equal @san - 4, @i3.reload.san
  end

  def test_neutral_repelling_test
    @gb.stubs( :repelling_roll ).returns( 0 )
    @gb.nyog_sothep_repelled_test
    assert_equal @san - 4, @i1.reload.san
    assert_equal @san - 4, @i2.reload.san
    assert_equal @san - 2, @i3.reload.san
  end

end

