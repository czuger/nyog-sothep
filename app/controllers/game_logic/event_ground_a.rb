module GameLogic
  module EventGroundA

    def table1_e2( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.current_loc = @last_location
      investigator.save!
    end

    def table1_e2( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :weapon, true )
    end

    def table1_e3( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :sign, true )
    end

    def table1_e4( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.see_psy!
    end

    def table1_e5( _ )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      PProfPosition.set_random_positions( @professor.current_loation, 3 )
    end

    def table1_e6( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :spell, true )
    end

    def table1_e7( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :weapon, false )
    end

    def table1_e9( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.update_attribute( :medaillon, true )
      investigator.play_again!
    end

    def table1_e10( _ )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      PProfPosition.set_random_positions( @professor.current_loation, 2 )
    end

    def table1_e11( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :weapon, true )
    end

    def table1_e12( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 3 )
      investigator.update_attribute( :spell, true )
    end

    def table1_e13( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      PProfPosition.delete_all
      PProfPosition.create!( position: @professor.current_location )
    end

    def table1_e14( investigator )
      EEventLog.log( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.play_again!
    end

    def table1_method_missing( _, _ )
      EEventLog.log( 'event non implementé' )
    end

  end
end

