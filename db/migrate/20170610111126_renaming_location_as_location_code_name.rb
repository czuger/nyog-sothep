class RenamingLocationAsLocationCodeName < ActiveRecord::Migration[5.0]
  def change

    rename_column :g_game_boards, :nyog_sothep_invocation_position, :nyog_sothep_invocation_position_code_name
    change_column_null :g_game_boards, :nyog_sothep_invocation_position_code_name, false

    rename_column :i_inv_target_positions, :position, :position_code_name


    rename_column :i_investigators, :current_location, :current_location_code_name

    rename_column :i_investigators, :last_location, :last_location_code_name

    rename_column :i_investigators, :ia_target_destination, :ia_target_destination_code_name

    rename_column :i_investigators, :forbidden_city, :forbidden_city_code_name


    rename_column :p_monster_positions, :location, :location_code_name

    rename_column :p_professors, :current_location, :current_location_code_name

  end
end
