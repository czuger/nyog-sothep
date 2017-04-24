class RemoveDeadFromIInvestigator < ActiveRecord::Migration[5.0]
  def change
    remove_column :i_investigators, :dead, :boolean
  end
end
