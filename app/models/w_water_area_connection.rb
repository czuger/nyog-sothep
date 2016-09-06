class WWaterAreaConnection < ApplicationRecord
  belongs_to :src_w_water_area, class_name: 'WWaterArea'
  belongs_to :dest_w_water_area, class_name: 'WWaterArea'
end
