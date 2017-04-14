module MapHelper

  def token_style( position, important: false )
    style = "top:#{position.y-17}px;left:#{position.x-17}px;transform:rotate(#{rand(-15..15)}deg);"
    style += 'z-index:10' if important
    style
  end

  def icon_div_id( elem )
    "#{elem}-#{@current_investigator.send( elem ) ? :enabled : :disabled }"
  end

  def can_breed_monster( monster )
    !@monster_at_prof_location && !@prof_in_water && !( monster.code_name == 'profonds' && !@prof_in_port )
  end

end
