require 'test_helper'

class PProfFightTest < ActiveSupport::TestCase

  # TODO : il faut séparer le check de qui attaque (investigateur/prof) et donc la décision du combat lui même

  def setup
    @gb = create( :g_game_board )
    @prof = @gb.p_professor
    @inv = create( :i_investigator, g_game_board: @gb, current_location_code_name: @prof.current_location_code_name )
  end

  test 'Investigator should attack professor and harm im 3 pv' do
    Kernel.stubs(:rand).returns(6)
    @inv.update( weapon: true )
    assert_no_difference '@inv.reload.san' do
      assert_difference '@prof.reload.hp', -3 do
        @prof.check_for_investigators_to_fight_in_city( @gb, prof_move: false )
      end
    end
  end

  test 'Investigator should attack professor and harm im 2 pv' do
    Kernel.stubs(:rand).returns(3)
    @inv.update( weapon: true )
    assert_no_difference '@inv.reload.san' do
      assert_difference '@prof.reload.hp', -2 do
        @prof.check_for_investigators_to_fight_in_city( @gb, prof_move: false )
      end
    end
  end

  test 'Investigator should attack professor and miss him' do
    Kernel.stubs(:rand).returns(1)
    @inv.update( weapon: true )
    assert_no_difference '@inv.reload.san' do
      assert_no_difference '@prof.reload.hp' do
        @prof.check_for_investigators_to_fight_in_city( @gb, prof_move: false )
      end
    end
  end

  test 'professor should not attack investigator' do
    [ [true, true], [true, false], [false, true] ].each do |weapon_medaillon_status|
      @inv.update( weapon: weapon_medaillon_status[0], medaillon: weapon_medaillon_status[1] )
      assert_no_difference '@inv.reload.san' do
        assert_no_difference '@prof.reload.hp' do
          @prof.check_for_investigators_to_fight_in_city( @gb )
        end
      end
    end
  end

  test 'professor should fight and investigator is protected by sign' do
    @inv.update( weapon: false, sign: true )
    assert_difference '@inv.reload.san', -2 do
      assert_no_difference '@prof.reload.hp' do
        @prof.check_for_investigators_to_fight_in_city( @gb )
      end
    end
    refute @inv.sign
  end

  test 'professor should fight and investigator is crushed' do
    @inv.update( weapon: false, sign: false )
    assert_difference '@inv.reload.san', -4 do
      assert_no_difference '@prof.reload.hp' do
        @prof.check_for_investigators_to_fight_in_city( @gb )
      end
    end
  end

end