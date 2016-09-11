module GameLogic
  module EventGroundB

    def table2_e2( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.see_psy!
    end

    def table2_e2( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 3 )
      investigator.update_attribute( :weapon, true )
    end

    def table2_e3( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :spell, true )
    end

    def table2_e4( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.play_again!
    end

    def table2_e5( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :sign, true )
    end

    def table2_e6( _ )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      PProfPosition.set_random_positions( @professor.current_location, 3 )
    end

    def table2_e7( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.current_location = @last_location
      investigator.save!
    end

    def table2_e8( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 3 )
      investigator.update_attribute( :medaillon, true )
    end

    def table2_e9( _ )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      PProfPosition.set_random_positions( @professor.current_location, 2 )
    end

    def table2_e10( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :spell, true )
      investigator.play_again!
    end

    def table2_e11( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.update_attribute( :weapon, true )
      investigator.play_again!
    end

    def table2_e12( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
    end

    def table2_e17( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.play_again!
    end

    def table2_e18( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :weapon, false )
    end

  end
end

