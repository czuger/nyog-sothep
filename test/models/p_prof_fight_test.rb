require 'test_helper'

class PProfFightTest < ActiveSupport::TestCase

  # TODO : il faut séparer le check de qui attaque (investigateur/prof) et donc la décision du combat lui même

  def setup
    @gb = create( :g_game_board )
    @prof = @gb.p_professor
    @inv = create( :i_investigator, g_game_board: @gb, current_location_code_name: @prof.current_location_code_name )
    @prof_position_finder = GameCore::Ia::ProfPositionFinder.new
  end

  test 'Investigator should attack professor and harm im 3 pv even if he has a medaillon' do
    Kernel.stubs(:rand).returns(6)
    @inv.update( weapon: true, medaillon: true )
    assert_no_difference '@inv.reload.san' do
      assert_difference '@prof.reload.hp', -3 do
        @inv.check_for_prof_to_fight_in_city( @gb, @prof, @prof_position_finder )
      end
    end
    assert_includes @prof_position_finder.get_prof_positions( @gb.turn ), @prof.current_location_code_name
  end

  test 'Investigator should attack professor and harm im 3 pv' do
    Kernel.stubs(:rand).returns(6)
    @inv.update( weapon: true )
    assert_no_difference '@inv.reload.san' do
      assert_difference '@prof.reload.hp', -3 do
        @inv.check_for_prof_to_fight_in_city( @gb, @prof, @prof_position_finder )
      end
    end
    assert_includes @prof_position_finder.get_prof_positions( @gb.turn ), @prof.current_location_code_name
  end

  test 'Investigator should attack professor and harm im 2 pv' do
    Kernel.stubs(:rand).returns(3)
    @inv.update( weapon: true )
    assert_no_difference '@inv.reload.san' do
      assert_difference '@prof.reload.hp', -2 do
        @inv.check_for_prof_to_fight_in_city( @gb, @prof, @prof_position_finder )
      end
    end
    assert_includes @prof_position_finder.get_prof_positions( @gb.turn ), @prof.current_location_code_name
  end

  test 'Investigator should attack professor and miss him' do
    Kernel.stubs(:rand).returns(1)
    @inv.update( weapon: true )
    assert_no_difference '@inv.reload.san' do
      assert_no_difference '@prof.reload.hp' do
        @inv.check_for_prof_to_fight_in_city( @gb, @prof, @prof_position_finder )
      end
    end
    assert_includes @prof_position_finder.get_prof_positions( @gb.turn ), @prof.current_location_code_name
  end

  test 'professor should not attack investigator' do
    @inv.update( weapon: true )
    assert_no_difference '@inv.reload.san' do
      assert_no_difference '@prof.reload.hp' do
        @prof.check_for_investigators_to_fight_in_city( @gb )
      end
    end
    refute @prof_position_finder.get_prof_positions( @gb.turn )
  end

  test 'professor should fight and investigator is protected by sign' do
    @inv.update( weapon: false, sign: true )
    assert_difference '@inv.reload.san', -2 do
      assert_no_difference '@prof.reload.hp' do
        @prof.check_for_investigators_to_fight_in_city( @gb )
      end
    end
    refute @inv.sign
    @prof_position_finder.load( @gb )
    assert_includes @prof_position_finder.get_prof_positions( @gb.turn ), @prof.current_location_code_name
  end

  test 'professor should fight and investigator is crushed' do
    @inv.update( weapon: false, sign: false )
    assert_difference '@inv.reload.san', -4 do
      assert_no_difference '@prof.reload.hp' do
        @prof.check_for_investigators_to_fight_in_city( @gb )
      end
    end
    @prof_position_finder.load( @gb )
    assert_includes @prof_position_finder.get_prof_positions( @gb.turn ), @prof.current_location_code_name
  end

end