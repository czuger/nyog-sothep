class CreateGDestroyedCities < ActiveRecord::Migration[5.0]
  def change
    create_table :g_destroyed_cities do |t|
      t.references :g_game_board, null: false, foreign_key: true
      t.string :city_code_name, null: false, index: { unique: true }
      t.integer :token_rotation, null: false

      t.timestamps
    end
  end
end