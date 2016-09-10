module GameLogicM
  class EventGround

    def self.e2( investigator )
      EEventLog.log( I18n.t( "events.#{__method__}.#{investigator.gender}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :weapon, true )
    end

    def self.e3( investigator )
      EEventLog.log( I18n.t( "events.#{__method__}.#{investigator.gender}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :medaillon, true )
    end

    def self.e4( investigator )
      EEventLog.log( I18n.t( "events.#{__method__}.#{investigator.gender}" ) )
      investigator.increment( :san, 5 )
      investigator.update_attribute( :delayed, true )
    end

    def self.e6( investigator )
      EEventLog.log( I18n.t( "events.#{__method__}.#{investigator.gender}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :spell, true )
    end

    def self.e7( investigator )
      EEventLog.log( I18n.t( "events.#{__method__}.#{investigator.gender}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :weapon, false )
    end

    def self.method_missing( _, _ )
      EEventLog.log( 'event non implementé' )
    end

  end
end

