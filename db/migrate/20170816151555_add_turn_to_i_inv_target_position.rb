class AddTurnToIInvTargetPosition < ActiveRecord::Migration[5.0]
  def change
    add_column :i_inv_target_positions, :turn, :integer, null: false
    remove_column :i_inv_target_positions, :memory_counter, :integer
  end
end
