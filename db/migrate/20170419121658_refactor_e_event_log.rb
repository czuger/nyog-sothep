class RefactorEEventLog < ActiveRecord::Migration[5.0]
  def change
    remove_column :e_event_logs, :logset, :integer
    remove_column :e_event_logs, :event, :string

    add_column :e_event_logs, :turn, :integer
    add_index :e_event_logs, :turn

    add_reference :e_event_logs, :actor, null: false, polymorphic: true, index: false
    add_column :e_event_logs, :actor_aasm_state, :string, null: false

    add_column :e_event_logs, :message, :string, null: false
  end
end
