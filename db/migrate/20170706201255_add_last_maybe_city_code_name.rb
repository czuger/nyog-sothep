class AddLastMaybeCityCodeName < ActiveRecord::Migration[5.0]
  def change
    add_column :p_professors, :last_fake_position_1_code_name, :string
    add_column :p_professors, :last_fake_position_2_code_name, :string
  end
end
