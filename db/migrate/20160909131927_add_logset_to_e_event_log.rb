class AddLogsetToEEventLog < ActiveRecord::Migration[5.0]

  def up
    execute <<-SQL
      CREATE SEQUENCE e_event_logs_logset;
    SQL

    add_column :e_event_logs, :logset, :integer, null: false
  end

  def down
    execute <<-SQL
      DROP SEQUENCE e_event_logs_logset;
    SQL

    remove_column :e_event_logs, :logset
  end

end
