class GDestroyedCity < ApplicationRecord

  def current_location
    GameCore::Map::Location.get_location( city_code_name )
  end

  def self.destroy_city( game_board, code_name )

    unless game_board.g_destroyed_cities.exists?( city_code_name: code_name )

      game_board.g_destroyed_cities.create!(
        city_code_name: code_name, token_rotation: rand( -15 .. 15 ) )

    end

  end

end
