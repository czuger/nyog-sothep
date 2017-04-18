class RemoveSpottedFromPProfessor < ActiveRecord::Migration[5.0]
  def change
    remove_column :p_professors, :spotted, :boolean
  end
end
