module MapsHelper

  def token_style
    "top:#{@zone.y-17}px;left:#{@zone.x-17}px;transform:rotate(#{rand(-5..5)}deg);"
  end

  def icon_div_id( elem )
    "#{elem}-#{@current_investigator.send( elem ) ? :enabled : :disabled }"
  end

end
