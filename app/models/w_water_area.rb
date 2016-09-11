class WWaterArea < ApplicationRecord
  has_many :ports, class_name: 'CCity'
  has_many :w_water_area_connections, foreign_key: :src_w_water_area_id
  has_many :connected_w_water_areas, through: :w_water_area_connections, source: :dest_w_water_area

  def destinations
    ports + connected_w_water_areas
  end

end
