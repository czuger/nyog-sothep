class AddNyogSothepInvokedToGGameBoard < ActiveRecord::Migration[5.0]
  def change
    add_column :g_game_boards, :nyog_sothep_invoked, :boolean
  end
end
