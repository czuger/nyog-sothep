module MapHelper

  def token_style( position, token_rotation = 0 )

    if @position_x_decal.has_key?( position.code_name )
      @position_x_decal[ position.code_name ] += @x_decal
    else
      @position_x_decal[ position.code_name ] = 0
    end
    x_decal = @position_x_decal[ position.code_name ]

    "top:#{position.y-17}px;left:#{position.x-17 + x_decal}px;transform:rotate(#{token_rotation}deg);"
  end

  def can_breed_monster( monster )
    !@monster_at_prof_location && !@prof_in_water && !( monster.code_name == 'profonds' && !@prof_in_port )
  end

  def investigator_info_string( investigator )
    "SAN : #{investigator.san}, weapon: #{investigator.weapon}, sign: #{investigator.sign}, medaillon: #{investigator.medaillon}, spell: #{investigator.spell}"
  end

end
