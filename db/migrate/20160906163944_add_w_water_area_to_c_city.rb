class AddWWaterAreaToCCity < ActiveRecord::Migration[5.0]
  def change
    add_reference :c_cities, :w_water_area, foreign_key: true, index: true
  end
end
