class CreateEEventLogSummaries < ActiveRecord::Migration[5.0]
  def change
    create_table :e_event_log_summaries do |t|

      t.references :g_game_board, foreign_key: true, null: false
      t.references :actor, null: false, index: nil, polymorphic: true

      t.integer :turn, null: false, default: 1

      t.string :event_translation_code, null: false
      t.string :event_translation_data, null: false

      t.references :e_event_log, foreign_key: true, null: false, index: nil

      t.timestamps
    end
  end
end
