class AddPlayersCountToGGameBoard < ActiveRecord::Migration[5.0]
  def change
    add_column :g_game_boards, :players_count, :integer, null: false
    add_column :g_game_boards, :ia_side, :string
    add_column :g_game_boards, :prof_security_code, :string, null: false
  end
end
