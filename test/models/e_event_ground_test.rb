require 'test_helper'

class EEventGroundTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board_with_event_ready_for_events_investigators )
    @investigator = @gb.reload.i_investigators.first
    @inv_san = @investigator.san
    @professor = @gb.p_professor
  end

  def test_great_psy_encounter
    IInvestigator.any_instance.stubs(:choose_table ).returns(1)
    IInvestigator.any_instance.stubs(:event_dices ).returns(4)
    @professor.update( current_location: CCity.find_by( code_name: :providence ) )

    assert @gb.reload.inv_events?
    GameCore::InvestigatorsActions.new( @gb, @gb.p_professor ).investigators_ia_play
    assert_equal @inv_san, @investigator.reload.san

    @gb.prof_movement_done!
    @gb.inv_movement_done!

    assert @gb.reload.inv_events?
    GameCore::InvestigatorsActions.new( @gb, @gb.p_professor ).investigators_ia_play
    assert_equal @inv_san+5, @investigator.reload.san
  end

  # def test_events_on_table_2
  #   @investigator.update( event_table: 2 )
  #   Kernel.stubs(:rand).returns(3)
  #   GameCore::Events.roll_event( @gb, @investigator, @professor )
  #   assert @investigator.reload.spell
  # end
  #
  # def test_no_event_on_water
  #   water_area = create( :w_water_area )
  #   @investigator.current_location = water_area
  #   @investigator.save!
  #   Kernel.stubs(:rand).returns(2)
  #   GameCore::Events.roll_event( @gb, @investigator, @professor )
  #   refute @investigator.reload.weapon
  # end
  #
  def test_all_event_grounds
    1.upto(18).each do |i|
      @gb.update( aasm_state: :inv_events )
      @investigator.update_attribute( :aasm_state, :events )
      @investigator.send( "table#{1}_e#{i}", @gb, @professor )

      @gb.update( aasm_state: :inv_events )
      @investigator.update_attribute( :aasm_state, :events )
      @investigator.send( "table#{2}_e#{i}", @gb, @professor )
    end
    # No assertion, this is just a pass through test
  end

end
