class AddAskedFakeCitiesCountToGGameBoard < ActiveRecord::Migration[5.0]
  def change
    add_column :g_game_boards, :asked_fake_cities_count, :integer
  end
end
