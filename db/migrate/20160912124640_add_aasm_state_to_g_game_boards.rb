class AddAasmStateToGGameBoards < ActiveRecord::Migration[5.0]
  def change
    add_column :g_game_boards, :aasm_state, :string, null: false
  end
end
