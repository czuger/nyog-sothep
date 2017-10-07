require 'test_helper'

class ProfPositionFinderTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board )
    @professor = @gb.p_professor
    @imt = GameCore::Ia::ProfPositionFinder.new()
  end

  test 'Remove different fake pos in the same turn' do
    @imt.add_fake_pos( 1, [ :foo, :bar ] )
    @imt.add_fake_pos( 1, [ :bar, :fooo ] )
    assert_equal [:bar], @imt.data[1][:fake_pos]
  end

  test 'Remove fake pos too far from previous spotted location' do
    @imt.spot_prof( 1, :woonsocket )
    @imt.add_fake_pos( 2, [ :milford, :ashland ] )
    assert_equal [:milford], @imt.data[2][:fake_pos]
  end

  test 'Remove fake pos too far from previous spotted location but keep previous spotted location (prof can stay)' do
    @imt.spot_prof( 1, :woonsocket )
    @imt.add_fake_pos( 2, [ :woonsocket, :ashland ] )
    assert_equal [:woonsocket], @imt.data[2][:fake_pos]
  end

  test 'Remove fake pos too far from previous fakes_positions' do
    @imt.add_fake_pos( 1, [ :milford, :taunton ] )
    @imt.add_fake_pos( 2, [ :ashland, :tremont ] )
    assert_equal [:ashland], @imt.data[2][:fake_pos]
  end

  test 'Remove fake pos too far from previous fakes positions but keep previous fakes positions (prof can stay)' do
    @imt.add_fake_pos( 1, [ :milford, :taunton ] )
    @imt.add_fake_pos( 2, [ :milford, :tremont ] )
    assert_equal [:milford], @imt.data[2][:fake_pos]
  end

end