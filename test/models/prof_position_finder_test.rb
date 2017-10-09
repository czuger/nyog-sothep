require 'test_helper'

class ProfPositionFinderTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board )
    @professor = @gb.p_professor
    @imt = GameCore::Ia::ProfPositionFinder.new()
  end

  test 'Check load and save' do
    @imt.load( @gb )
    @imt.add_fake_pos( 1, @professor,[ 'foo', 'bar' ] )
    @imt.save( @gb )
    @imt = nil
    @imt = GameCore::Ia::ProfPositionFinder.new()
    @imt.load( @gb )
    assert_equal [ 'foo', 'bar', 'providence' ], @imt.data[1][:fake_pos]
  end

  test 'Remove different fake pos in the same turn' do
    @imt.add_fake_pos( 1, @professor,[ 'foo', 'bar' ] )
    @imt.add_fake_pos( 1, @professor,[ 'bar', 'fooo' ] )
    assert_equal ['bar', 'providence'], @imt.data[1][:fake_pos]
  end

  test 'Remove fake pos too far from previous spotted location' do
    @imt.spot_prof( 1, 'woonsocket' )
    @imt.add_fake_pos( 2, @professor,[ 'milford', 'ashland' ] )
    assert_equal ['milford', 'providence'], @imt.data[2][:fake_pos]
  end

  test 'Remove fake pos too far from previous spotted location but keep previous spotted location (prof can stay)' do
    @imt.spot_prof( 1, 'providence' )
    @imt.add_fake_pos( 2, @professor,%w( ashland woonsocket ) )
    assert_equal %w( woonsocket providence ), @imt.get_prof_positions( 2 )
  end

  test 'Remove fake pos too far from previous fakes_positions' do
    @imt.add_fake_pos( 1, @professor,%w( milford taunton ) )
    assert_equal %w( milford taunton providence ), @imt.get_prof_positions( 1 )
    @professor.update( current_location_code_name: 'woonsocket' )
    @imt.add_fake_pos( 2, @professor,%w( ashland tremont ) )
    assert_equal %w( ashland woonsocket ), @imt.get_prof_positions( 2 )
  end

  test 'Remove fake pos too far from previous fakes positions but keep previous fakes positions (prof can stay)' do
    @imt.add_fake_pos( 1, @professor,[ 'milford', 'taunton' ] )
    @imt.add_fake_pos( 2, @professor,[ 'milford', 'tremont' ] )
    assert_equal ['milford', 'providence'], @imt.data[2][:fake_pos]
  end

end