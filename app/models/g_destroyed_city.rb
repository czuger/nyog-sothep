class GDestroyedCity < ApplicationRecord

  def current_location
    GameCore::Map::Location.get_location( city_code_name )
  end

end
