require 'test_helper'

class PProfFightTest < ActiveSupport::TestCase

  # TODO : il faut séparer le check de qui attaque (investigateur/prof) et donc la décision du combat lui même

  def setup
    @gb = create( :g_game_board )
    @prof = @gb.p_professor
    @inv = create( :i_investigator, g_game_board: @gb, current_location_code_name: @prof.current_location_code_name )
  end

  test 'professor should attack investigator and loose 3 pv' do
    Kernel.stubs(:rand).returns(6)
    @inv.update_attribute( :weapon, true )
    assert_no_difference '@inv.san' do
      assert_difference '@prof.hp', -3 do
        @prof.check_for_investigators_to_fight_in_city( @gb )
      end
    end
  end

  test 'professor should attack investigator and loose 2 pv' do
    Kernel.stubs(:rand).returns(3)
    @inv.update_attribute( weapon: true )
    assert_difference '@prof.hp', -2 do
      @prof.check_for_investigators_to_fight_in_city( @game_board )
    end
  end

  test 'professor should attack investigator and miss him' do
    Kernel.stubs(:rand).returns(1)
    @inv.update_attribute( weapon: true )
    assert_difference '@prof.hp', -2 do
      @prof.check_for_investigators_to_fight_in_city( @game_board )
    end
  end

  test 'professor should fight and investigator is protected by sign' do
    @investigator.update( weapon: false, sign: true )
    @prof.check_for_investigators_to_fight_in_city( @game_board )
    assert_redirected_to g_game_board_play_url( attacking_investigator_id: @investigator.id )
  end

  test 'professor should fight and investigator is crushed' do
    @investigator.update( weapon: false, sign: false )
    @prof.check_for_investigators_to_fight_in_city( @game_board )
    assert_redirected_to g_game_board_play_url( attacking_investigator_id: @investigator.id )
  end

end