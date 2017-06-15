class NyogSothepInvokedBecomesMandatory < ActiveRecord::Migration[5.0]
  def change
    change_column_null :g_game_boards, :nyog_sothep_invoked, false
    change_column :g_game_boards, :nyog_sothep_invoked, :boolean, :default => false
  end
end
