require 'test_helper'

class BfsAlgoTest < ActiveSupport::TestCase

  def setup
  end

  # test 'same location' do
  #   assert_raise do
  #     GameCore::Ia::BfsAlgo.find_next_dest_to_goal( :oxford, :oxford )
  #   end
  # end

  test 'should return nil if destination unreachable' do
    destroyed_cities_codes_names = [ 'oxford', 'woonsocket' ]
    result, _ = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( :pascoag, :milford,
                                                              destroyed_cities_codes_names )
    refute result
  end

  test 'one step' do
    result, _ = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( :oxford, :providence )
    assert_equal :plainfield, result
  end

  test 'oxford -> innsmouth' do
    result, _ = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( :oxford, :innsmouth )
    assert_equal :plainfield, result
  end

  test 'algo should avoid plainfield to go to providence. Then next step should be pascoag' do
    src_city = :oxford
    dest_city = :providence
    forbidden_city = 'plainfield'
    should_go = :pascoag
    result, _ = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( src_city, dest_city,
                                                              [ forbidden_city ] )
    assert_equal should_go, result
  end

  test 'distance between oxford and insmouth should be 7' do
    _, distance = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( :oxford, :innsmouth )
    assert_equal 6, distance
  end

  test 'distance between worcester and lowell should be 4' do
    _, distance = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( :worcester, :lowell )
    assert_equal 3, distance
  end

  test 'one step distance city' do
    cities = GameCore::Ia::BfsAlgo.find_cities_around_city( :taunton, 2 )
    assert_includes cities[2], :milford
  end

  test '3 setp distance city from water area' do
    cities = GameCore::Ia::BfsAlgo.find_cities_around_city( :nantucket_sound, 3 )
    assert_includes cities[2], :barnstable
    refute_includes cities[1], :buzzards_bay
    refute_includes cities[2], :buzzards_bay
    refute_includes cities[3], :buzzards_bay
  end

  test '3 setp distance city from nantucket' do
    cities = GameCore::Ia::BfsAlgo.find_cities_around_city( :nantucket, 3 )
    # p cities
    assert_empty cities[1]
    refute_includes cities[2], :buzzards_bay
    assert_includes cities[2], :chatham
    refute_includes cities[3], :buzzards_bay
    assert_includes cities[3], :innsmouth
  end

end