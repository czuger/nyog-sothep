class RemoveProfSecurityCodeFromGGameBoard < ActiveRecord::Migration[5.0]
  def change
    remove_column :g_game_boards, :prof_security_code, :string
  end
end
