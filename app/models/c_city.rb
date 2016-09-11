class CCity < ApplicationRecord
  belongs_to :w_water_area, optional: true
  has_many :outgoing_r_roads, foreign_key: :src_city_id, class_name: 'RRoad'
  has_many :dest_cities, through: :outgoing_r_roads

  has_many :people, as: :l_location

  def destinations
    ( dest_cities + [ loc.w_water_area ] ).comact
  end

end
