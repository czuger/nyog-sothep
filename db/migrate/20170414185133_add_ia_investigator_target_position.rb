class AddIaInvestigatorTargetPosition < ActiveRecord::Migration[5.0]
  def change
    drop_table :p_prof_positions

    create_table :i_inv_target_position do |t|

      t.references :g_game_board
      t.references :position, null: false, polymorphic: true, index: false

      t.integer :memory_counter, default: 3, null: false

      t.timestamps
    end

  end
end
