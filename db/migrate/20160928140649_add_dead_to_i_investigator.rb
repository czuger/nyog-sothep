class AddDeadToIInvestigator < ActiveRecord::Migration[5.0]
  def change
    add_column :i_investigators, :dead, :boolean, null: false, default: false
  end
end
