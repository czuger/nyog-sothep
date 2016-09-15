class AddSpottedToPProfessor < ActiveRecord::Migration[5.0]
  def change
    add_column :p_professors, :spotted, :boolean, default: false, null: false
  end
end
