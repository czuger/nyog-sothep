class CreatePMonsterPositions < ActiveRecord::Migration[5.0]
  def change
    create_table :p_monster_positions do |t|
      t.references :location, null: false, polymorphic: true
      t.string :code_name, null: false
      t.boolean :discovered, null: false, default: false

      t.timestamps
    end
  end
end
