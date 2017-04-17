module MapHelper

  def token_style( position, token_rotation = 0 )
    "top:#{position.y-17}px;left:#{position.x-17}px;transform:rotate(#{token_rotation}deg);"
  end

  def icon_div_id( elem )
    "#{elem}-#{@current_investigator.send( elem ) ? :enabled : :disabled }"
  end

  def can_breed_monster( monster )
    !@monster_at_prof_location && !@prof_in_water && !( monster.code_name == 'profonds' && !@prof_in_port )
  end

end
