class CreateCCities < ActiveRecord::Migration[5.0]
  def change
    create_table :c_cities do |t|
      t.string :code_name, null: false
      t.integer :x, null: false
      t.integer :y, null: false

      t.timestamps
    end
    add_index :c_cities, :code_name, unique: true
  end
end
