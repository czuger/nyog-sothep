class LogsRefactoring < ActiveRecord::Migration[5.0]
  def change

    create_table :l_logs do |t|
      t.references :g_game_board, null: false, foreign_key: true

      t.integer :turn, null: false

      t.references :actor, polymorphic: true
      t.string :actor_aasm_state

      t.boolean :summary

      t.string :event_translation_code, null: false
      t.string :event_translation_data, null: false

      t.timestamps
    end
  end
end
