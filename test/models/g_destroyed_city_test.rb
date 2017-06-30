require 'test_helper'

class GDestroyedCityTest < ActiveSupport::TestCase

  def setup
    @gb = create( :g_game_board )
  end

  def test_alive_investigators_should_select_only_living_investigators
    GDestroyedCity.destroy_city( @gb, :dunwich )
  end

end
