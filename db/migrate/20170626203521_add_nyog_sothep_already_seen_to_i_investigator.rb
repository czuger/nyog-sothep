class AddNyogSothepAlreadySeenToIInvestigator < ActiveRecord::Migration[5.0]
  def change
    add_column :i_investigators, :nyog_sothep_already_seen, :boolean, null: false, default: false
  end
end
