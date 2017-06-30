class PMonsterPosition < ApplicationRecord

  belongs_to :g_game_board

  def current_location
    GameCore::Map::Location.get_location( location_code_name )
  end

end
