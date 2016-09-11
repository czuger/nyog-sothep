class CreatePProfPositions < ActiveRecord::Migration[5.0]
  def change
    create_table :p_prof_positions do |t|
      t.references :position, null: false, polymorphic: true

      t.timestamps
    end
  end
end
