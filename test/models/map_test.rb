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


end
