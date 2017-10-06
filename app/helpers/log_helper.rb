module LogHelper

  def log_message( log, summary: false )

    params = log.event_translation_data
    translation_code = log.event_translation_code
    act = log.actor

    if act&.is_a?( IInvestigator )
      if summary
        params[ :investigator_name ] = act.translated_name( 'beginning' )
      else
        params[ :investigator_name ] = ( log.name_translation_method ? act.translated_name( log.name_translation_method ) : act.translated_name )
      end
    end

    if summary
      translation_code += '.log_summary'
      translation_code += ('.' + act.gender) if log.log_gender_summary
    else
      translation_code += '.log'
      translation_code += ('.' + act.gender) if log.log_gender
    end

    if params[ :dest_cn ]
      params[ :dest_cn ] = I18n.t( 'movement.locations.' + params[ :dest_cn ].to_s )
    end

    I18n.t( translation_code, params )

  end

end
