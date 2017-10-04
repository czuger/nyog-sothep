class RemoveSummaryFromLLog < ActiveRecord::Migration[5.0]
  def change
    remove_column :l_logs, :summary, :boolean
    add_column :l_logs, :event_translation_summary_code, :string
  end
end
