module MapHelper

  def token_style( position, token_rotation = 0 )

    if @position_x_decal.has_key?( position.code_name )
      @position_x_decal[ position.code_name ] += @x_decal
    else
      @position_x_decal[ position.code_name ] = 0
    end
    x_decal = @position_x_decal[ position.code_name ]

    "top:#{position.y-17+13}px;left:#{position.x-17 + x_decal}px;transform:rotate(#{token_rotation}deg);z-index:#{x_decal}"
  end

  def can_breed_monster( monster )
    !@monster_at_prof_location && !@prof_in_water && !( monster.code_name == 'profonds' && !@prof_in_port )
  end

  def nyog_sothep_class()
    unless @game_board.nyog_sothep_invoked
      return 'nyog-sothep-hidden'
    end
    ''
  end

  # Investigator part

  def investigator_info_string( investigator )
    result = "#{investigator.translated_name} (SAN #{investigator.san})<br>"
    result << image_tag( 'tokens/weapon.jpg', class: 'investigator-inventory-symlbol' ) if investigator.weapon
    result << image_tag( 'tokens/sign.jpg', class: 'investigator-inventory-symlbol' ) if investigator.sign
    result << image_tag( 'tokens/medaillon.jpg', class: 'investigator-inventory-symlbol' ) if investigator.medaillon
    result << image_tag( 'tokens/nyogsothep_s.jpg', class: 'investigator-inventory-symlbol' ) if investigator.spell

    # : , weapon: #{investigator.weapon}, sign: #{investigator.sign}, medaillon: #{investigator.medaillon}, spell: #{investigator.spell}"
    result
  end

  def investigator_info_style( position )
    x_decal = @position_x_decal[ position.code_name ]

    "top:#{position.y-17+13-50-15}px;left:#{position.x-17-30 + x_decal}px;z-index:50"
  end

  # Log summary part

  def event_log_summary( log_summary )

    name = log_summary.actor.translated_name
    params = log_summary.event_translation_data
    params[ :investigator_name ] = name

    result = 'Not implemented'

    if params.has_key?( :san_loss )
      result = I18n.t( 'log_summary.loose_san', params )
    elsif params.has_key?( :san_gain )
      result = I18n.t( 'log_summary.gain_san', params )
    end

    result
  end

end
