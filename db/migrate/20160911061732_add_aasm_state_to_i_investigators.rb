class AddAasmStateToIInvestigators < ActiveRecord::Migration
  def change
    add_column :i_investigators, :aasm_state, :string, default: :normal
    remove_column :i_investigators, :delayed
  end
end
