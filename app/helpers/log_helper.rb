module LogHelper

  def log_message( log )

    params = log.event_translation_data
    translation_code = log.event_translation_code

    if log.actor.is_a?( IInvestigator )
      act = log.actor
      params[ :investigator_name ] = ( log.name_translation_method ? act.translated_name( log.name_translation_method ) : act.translated_name )
      translation_code += '.' + act.gender
    end

    I18n.t( translation_code, params )

  end
end
