class AddLogGenderToLLog < ActiveRecord::Migration[5.0]
  def change
    add_column :l_logs, :log_gender, :boolean, default: false
    add_column :l_logs, :log_gender_summary, :boolean, default: false
    add_column :l_logs, :summary, :boolean, default: false
    add_index :l_logs, :summary
    remove_column :l_logs, :event_translation_summary_code, :string
  end
end
