require 'test_helper'

class InvestigatorChooseBestCityToMeetTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board )
    create( :i_investigator, g_game_board_id: @gb.id, current_location_code_name: :plainfield )
    create( :i_investigator, g_game_board_id: @gb.id, current_location_code_name: :plainfield  )
    create( :g_destroyed_city, g_game_board_id: @gb.id, city_code_name: :worcester )
    create( :g_destroyed_city, g_game_board_id: @gb.id, city_code_name: :pascoag )
    create( :g_destroyed_city, g_game_board_id: @gb.id, city_code_name: :providence )
    create( :g_destroyed_city, g_game_board_id: @gb.id, city_code_name: :oxford )

    @finder = GameCore::Ia::InvestigatorChooseBestCityToMeet.new( @gb )
  end

  test 'should find a path' do
    assert @finder.get_best_city
  end

  test 'should not find a path' do
    create( :g_destroyed_city, g_game_board_id: @gb.id, city_code_name: :westerly )
    refute @finder.get_best_city
  end

end