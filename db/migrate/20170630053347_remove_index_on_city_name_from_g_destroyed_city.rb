class RemoveIndexOnCityNameFromGDestroyedCity < ActiveRecord::Migration[5.0]
  def change
    remove_index :g_destroyed_cities, :city_code_name
  end
end
