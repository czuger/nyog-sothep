class RemovingLocationsInDatabase < ActiveRecord::Migration[5.0]
  def change

    remove_reference :g_game_boards, :nyog_sothep_invocation_position
    add_column :g_game_boards, :nyog_sothep_invocation_position, :string, null: false

    remove_reference :i_inv_target_positions, :position
    add_column :i_inv_target_positions, :position, :string, null: false


    remove_reference :i_investigators, :current_location
    add_column :i_investigators, :current_location, :string, null: false

    remove_reference :i_investigators, :last_location
    add_column :i_investigators, :last_location, :string, null: false

    remove_reference :i_investigators, :ia_target_destination
    add_column :i_investigators, :ia_target_destination, :string, null: false

    remove_column :i_investigators, :forbidden_city_id
    add_column :i_investigators, :forbidden_city, :string


    remove_reference :p_monster_positions, :location
    add_column :p_monster_positions, :location, :string, null: false

    remove_reference :p_professors, :current_location
    add_column :p_professors, :current_location, :string, null: false

    drop_table :r_roads
    drop_table :c_cities

    drop_table :w_water_area_connections
    drop_table :w_water_areas

  end
end
