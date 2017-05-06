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

  test 'algo should avoid plainfield to go to providence. Then next step should be pascoag' do
    create( :oxford_pascoag )
    create( :pascoag_woonsocket )
    create( :woonsocket_providence )
    create( :plainfield_to_providence )
    src_city = CCity.find_by( code_name: :oxford )
    dest_city = CCity.find_by( code_name: :providence )
    forbidden_city = CCity.find_by( code_name: :plainfield )
    should_go = CCity.find_by( code_name: :pascoag )
    result = GameCore::Ia::BfsAlgo.find_next_dest_to_goal( src_city, dest_city, forbidden_city: forbidden_city )
    assert_equal should_go, result
  end

end