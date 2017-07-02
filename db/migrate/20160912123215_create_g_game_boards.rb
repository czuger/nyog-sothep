class CreateGGameBoards < ActiveRecord::Migration[5.0]
  def change
    create_table :g_game_boards do |t|
      t.integer :turn

      t.timestamps
    end

    add_column :i_investigators, :g_game_board_id, :integer, null: false
    add_foreign_key :i_investigators, :g_game_boards
    add_index :i_investigators, :g_game_board_id

    add_column :m_monsters, :g_game_board_id, :integer, null: false
    add_foreign_key :m_monsters, :g_game_boards
    add_index :m_monsters, :g_game_board_id

    add_column :p_monster_positions, :g_game_board_id, :integer, null: false
    add_foreign_key :p_monster_positions, :g_game_boards
    add_index :p_monster_positions, :g_game_board_id

    add_column :p_monsters, :g_game_board_id, :integer, null: false
    add_foreign_key :p_monsters, :g_game_boards
    add_index :p_monsters, :g_game_board_id

    add_column :p_prof_positions, :g_game_board_id, :integer, null: false
    add_foreign_key :p_prof_positions, :g_game_boards
    add_index :p_prof_positions, :g_game_board_id

    add_column :p_professors, :g_game_board_id, :integer, null: false
    add_foreign_key :p_professors, :g_game_boards
    add_index :p_professors, :g_game_board_id
    
  end
end
