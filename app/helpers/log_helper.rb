module LogHelper

  def log_message( log, summary: false )

    params = log.event_translation_data
    translation_code = summary ? log.event_translation_summary_code : log.event_translation_code

    act = log.actor

    if act.is_a?( IInvestigator )
      params[ :investigator_name ] = ( log.name_translation_method ? act.translated_name( log.name_translation_method ) : act.translated_name )
    end

    translation_code += '.' + act.gender

    I18n.t( translation_code, params )

  end

end
