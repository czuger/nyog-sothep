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

  def investigator_info_string( investigator )
    "#{investigator.translated_name}, SAN : #{investigator.san}, weapon: #{investigator.weapon}, sign: #{investigator.sign}, medaillon: #{investigator.medaillon}, spell: #{investigator.spell}"
  end

  def nyog_sothep_class()
    unless @game_board.nyog_sothep_invoked
      return 'nyog-sothep-hidden'
    end
    ''
  end

  def event_log_summary_translation_param( log_summary )
    base_hash = log_summary.event_translation_data
    base_hash[ :investigator_name ] = log_summary.actor.translated_name
    base_hash[ :tour_no ] = log_summary.turn
    base_hash
  end

end
