require 'test_helper'

class BfsAlgoTest < ActiveSupport::TestCase

  def setup
  end

  test 'same location' do
    assert_raise do
      GameCore::Ia::BfsAlgo.find_next_dest_to_goal( :oxford, :oxford )
    end
  end

  test 'one step' do
    result = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( :oxford, :providence )
    assert_equal :plainfield, result
  end

  test 'oxford -> innsmouth' do
    result = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( :oxford, :innsmouth )
    assert_equal :plainfield, result
  end

  test 'algo should avoid plainfield to go to providence. Then next step should be pascoag' do
    src_city = :oxford
    dest_city = :providence
    forbidden_city = :plainfield
    should_go = :pascoag
    result = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( src_city, dest_city, forbidden_city )
    assert_equal should_go, result
  end

end