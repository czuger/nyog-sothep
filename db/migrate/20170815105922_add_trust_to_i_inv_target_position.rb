class AddTrustToIInvTargetPosition < ActiveRecord::Migration[5.0]
  def change
    add_column :i_inv_target_positions, :trust, :float, null: false, default: 0.5
    change_column_default :i_inv_target_positions, :memory_counter, 0
  end
end