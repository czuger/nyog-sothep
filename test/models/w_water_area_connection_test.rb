require 'test_helper'

class WWaterAreaConnectionTest < ActiveSupport::TestCase

  test 'Verify area connections' do

    conn = create( :w_water_area_connection )
    src = conn.src_w_water_area
    dest = conn.dest_w_water_area

    assert_includes src.connected_w_water_areas.to_a, dest

  end

end
