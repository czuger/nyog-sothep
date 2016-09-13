FactoryGirl.define do
  factory :w_water_area_connection do
    src_w_water_area { create( :w_water_area ) }
    dest_w_water_area { create( :w_water_area ) }
  end
end
