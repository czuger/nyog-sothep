require 'test_helper'

class MapTest < ActiveSupport::TestCase

  def test_map_has_port
    location = GameCore::Map::Location.get_location( :boston )
    assert location.port?
  end

  def test_all_cities
    cities = GameCore::Map::City.all
    assert_equal 47, cities.count
  end

  def test_destinations
    oxford = GameCore::Map::Location.get_location( :oxford )
    destinations_code_names = oxford.destinations.map{ |e| e.id }
    assert_equal [:plainfield, :pascoag, :worcester], destinations_code_names
  end

  def test_city_should_not_fail_if_requeted_for_water_area
    oxford = GameCore::Map::Location.get_location( :oxford )
    refute oxford.water_area?
  end

  def test_borders_crossings
    assert GameCore::Map::BordersCrossings.check?( :dunwich, :nashua )
    assert GameCore::Map::BordersCrossings.check?( :providence, 'taunton' )
    refute GameCore::Map::BordersCrossings.check?( :middleboro, 'tremont' )
  end

end
