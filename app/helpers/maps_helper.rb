module MapsHelper

  def token_style
    "top:#{@zone.y-17}px;left:#{@zone.x-17}px;transform:rotate(#{rand(-5..5)}deg);"
  end

end
