class CreatePProfessors < ActiveRecord::Migration[5.0]
  def change
    create_table :p_professors do |t|
      t.integer :hp, null: false
      t.references :current_location, polymorphic: true, index: false, null: false

      t.timestamps
    end
  end
end
