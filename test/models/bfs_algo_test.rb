require 'test_helper'

class BfsAlgoTest < ActiveSupport::TestCase

  def setup
    @r = create( :true_road )
  end

  test 'same location' do
    assert_raise do
      GameCore::Ia::BfsAlgo.find_next_dest_to_goal( @r.src_city, @r.src_city )
    end
  end

  test 'one step' do
    result = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( @r.src_city, @r.dest_city )
    assert_equal @r.dest_city, result
  end

end