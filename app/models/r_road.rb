class RRoad < ApplicationRecord
  belongs_to :src_city, class_name: 'CCity'
  belongs_to :dest_city, class_name: 'CCity'
end
