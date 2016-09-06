class CreateRRoutes < ActiveRecord::Migration[5.0]
  def change
    create_table :r_roads do |t|
      t.references :src_city, null: false
      t.references :dest_city, null: false
      t.boolean :border

      t.timestamps
    end
    add_foreign_key :r_roads, :c_cities, column: :src_city_id
    add_foreign_key :r_roads, :c_cities, column: :dest_city_id
    add_index :r_roads, [ :src_city_id, :dest_city_id ], unique: true
  end
end
