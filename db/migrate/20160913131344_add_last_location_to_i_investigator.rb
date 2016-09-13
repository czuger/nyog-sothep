class AddLastLocationToIInvestigator < ActiveRecord::Migration[5.0]
  def change
    add_reference :i_investigators, :last_location, null: false, polymorphic: true, index: false
  end
end
