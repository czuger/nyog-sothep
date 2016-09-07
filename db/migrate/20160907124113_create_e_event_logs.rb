class CreateEEventLogs < ActiveRecord::Migration[5.0]
  def change
    create_table :e_event_logs do |t|
      t.string :event

      t.timestamps
    end
  end
end
