class AddNyogSothepInvocationPositionToGGameBoard < ActiveRecord::Migration[5.0]
  def change
    add_reference :g_game_boards, :nyog_sothep_invocation_position, foreign_key: { to_table: :c_cities }, index: false
    add_column :g_game_boards, :nyog_sothep_invocation_position_rotation, :integer, null: false
  end
end
