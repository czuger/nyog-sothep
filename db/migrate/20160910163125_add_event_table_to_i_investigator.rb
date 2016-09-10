class AddEventTableToIInvestigator < ActiveRecord::Migration[5.0]
  def change
    add_column :i_investigators, :event_table, :integer, null: false, default: 1
  end
end
