class CreateIInvestigators < ActiveRecord::Migration[5.0]
  def change
    create_table :i_investigators do |t|
      t.string :code_name, null: false
      t.integer :san, null: false
      t.boolean :delayed, null: false, default: false
      t.boolean :weapon, null: false, default: false
      t.boolean :medaillon, null: false, default: false
      t.boolean :sign, null: false, default: false
      t.boolean :spell, null: false, default: false
      t.boolean :current, index: true, null: false, default: false
      t.references :current_location, polymorphic: true, index: false, null: false

      t.timestamps
    end
  end
end
