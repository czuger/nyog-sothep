class AddPortToCCity < ActiveRecord::Migration[5.0]
  def change
    add_column :c_cities, :port, :boolean
  end
end
