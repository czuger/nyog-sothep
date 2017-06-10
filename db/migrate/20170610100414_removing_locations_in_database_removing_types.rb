class RemovingLocationsInDatabaseRemovingTypes < ActiveRecord::Migration[5.0]
  def change

    remove_column :i_inv_target_positions, :position_type
    
    
    remove_column :i_investigators, :current_location_type
    

    remove_column :i_investigators, :last_location_type
    

    remove_column :i_investigators, :ia_target_destination_type
    

    remove_column :p_monster_positions, :location_type
    

    remove_column :p_professors, :current_location_type
    
  end
end
