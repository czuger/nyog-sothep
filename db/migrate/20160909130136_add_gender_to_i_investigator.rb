class AddGenderToInvestigator < ActiveRecord::Migration[5.0]
  def change
    add_column :i_investigators, :gender, :string, null: false, limit: 1
  end
end
