class RenameTokenOrientation < ActiveRecord::Migration[5.0]
  def change
    rename_column :p_monster_positions, :token_orientation, :token_rotation
    rename_column :p_professors, :token_orientation, :token_rotation
    rename_column :i_investigators, :token_orientation, :token_rotation
  end
end
