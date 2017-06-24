class AddNyogSothepRepellingCityCodeName < ActiveRecord::Migration[5.0]
  def change
    add_column :g_game_boards, :nyog_sothep_repelling_city_code_name, :string
  end
end
