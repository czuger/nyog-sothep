class RenameIaInvestigatorTargetPosition < ActiveRecord::Migration[5.0]
  def change
    rename_table :i_inv_target_position, :i_inv_target_positions
  end
end
