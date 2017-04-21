class RemoveIaSideFromGGameBoard < ActiveRecord::Migration[5.0]
  def change
    remove_column :g_game_boards, :ia_side, :string
  end
end
