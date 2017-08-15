class RemoveEventTableFromIInvestigator < ActiveRecord::Migration[5.0]
  def change
    remove_column :i_investigators, :event_table, :integer
  end
end
