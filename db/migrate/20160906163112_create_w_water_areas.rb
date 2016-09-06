class CreateWWaterAreas < ActiveRecord::Migration[5.0]
  def change
    create_table :w_water_areas do |t|
      t.string :code_name
      t.integer :x
      t.integer :y

      t.timestamps
    end
  end
end
