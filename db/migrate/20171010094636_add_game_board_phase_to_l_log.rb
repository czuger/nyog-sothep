class AddGameBoardPhaseToLLog < ActiveRecord::Migration[5.0]
  def change
    add_column :l_logs, :game_board_phase, :string, null: false
  end
end
