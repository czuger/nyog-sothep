class AddTokenOrientationToPMonsterPosition < ActiveRecord::Migration[5.0]
  def change
    add_column :p_monster_positions, :token_orientation, :integer, null: false
    add_column :p_professors, :token_orientation, :integer, null: false
    add_column :i_investigators, :token_orientation, :integer, null: false
  end
end
