class AddAasmStateToIInvestigators < ActiveRecord::Migration[5.0]
  def change
    add_column :i_investigators, :aasm_state, :string, null: false
    remove_column :i_investigators, :delayed
  end
end
