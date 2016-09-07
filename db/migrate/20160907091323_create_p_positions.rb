class CreatePPositions < ActiveRecord::Migration[5.0]
  def change
    create_table :p_positions do |t|
      t.string :code_name, null: false
      t.references :l_location, null: false, polymorphic: true
      t.boolean :current, null: false, default: false, index: true

      t.timestamps
    end
  end
end
