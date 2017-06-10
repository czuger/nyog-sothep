class PMonsterPosition < ApplicationRecord

  def current_location
    GameCore::Map::Location.get_location( location_code_name )
  end

end
