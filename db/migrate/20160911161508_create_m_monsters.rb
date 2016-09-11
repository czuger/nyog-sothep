class CreateMMonsters < ActiveRecord::Migration[5.0]
  def change
    create_table :m_monsters do |t|
      t.string :code_name, null: false

      t.timestamps
    end
  end
end
