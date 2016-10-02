json.extract! g_game_board, :id, :players_count, :ia_side, :created_at, :updated_at
json.url g_game_board_url(g_game_board, format: :json)