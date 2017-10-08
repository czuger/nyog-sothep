class CreatePProfPositionFinderStorages < ActiveRecord::Migration[5.0]

  def change
    drop_table :i_inv_target_positions do
    end

    create_table :ia_prof_positions do |t|
      t.references :g_game_board, foreign_key: true
      t.string :gb_data

      t.timestamps
    end
  end
end
