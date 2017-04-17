class SetTurnNotNullInGGameBoard < ActiveRecord::Migration[5.0]
  def change
    change_column_null :g_game_boards, :turn, false
  end
end
