class AddIaTargetDestinationToIInvestigator < ActiveRecord::Migration[5.0]
  def change
    add_reference :i_investigators, :ia_target_destination, polymorphic: true, index: false
  end
end
