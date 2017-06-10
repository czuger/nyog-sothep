require 'test_helper'

class MapTest < ActiveSupport::TestCase

  def test_map_has_port
    location = GameCore::Map::Location.get_location( :boston )
    assert location.port?
  end

  def test_all_cities
    cities = GameCore::Map::City.all
    assert_equal 53, cities.count
  end

  def test_destinations
    oxford = GameCore::Map::Location.get_location( :oxford )
    destinations_code_names = oxford.destinations.map{ |e| e.id }
    assert_equal [:plainfield, :pascoag, :worcester], destinations_code_names
  end


end
