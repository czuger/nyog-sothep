class AddNyogSothepProfLinkedToGGameBoard < ActiveRecord::Migration[5.0]
  def change
    add_column :g_game_boards, :nyog_sothep_prof_linked, :boolean, default: false, null: false
  end
end
