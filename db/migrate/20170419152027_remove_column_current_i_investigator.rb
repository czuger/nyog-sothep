class RemoveColumnCurrentIInvestigator < ActiveRecord::Migration[5.0]
  def change
    remove_column :i_investigators, :current, boolean: false
  end
end
