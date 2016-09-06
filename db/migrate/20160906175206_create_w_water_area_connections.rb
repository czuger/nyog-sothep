class CreateWWaterAreaConnections < ActiveRecord::Migration[5.0]
  def change
    create_table :w_water_area_connections do |t|
      t.references :src_w_water_area
      t.references :dest_w_water_area

      t.timestamps
    end
    add_foreign_key :w_water_area_connections, :w_water_areas, column: :src_w_water_area_id
    add_foreign_key :w_water_area_connections, :w_water_areas, column: :dest_w_water_area_id
    add_index :w_water_area_connections, [ :src_w_water_area_id, :dest_w_water_area_id ], unique: true, name: :w_water_area_connections_id
  end
end
