module MapsHelper

  def token_style
    "top:#{@zone.y-17}px;left:#{@zone.x-17}px;transform:rotate(#{rand(-15..15)}deg);"
  end

  def icon_div_id( elem )
    "#{elem}-#{@current_investigator.send( elem ) ? :enabled : :disabled }"
  end

end
