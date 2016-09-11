module GameLogicM
  class EventGroundA

    def self.e2( investigator )
      EEventLog.log( I18n.t( "events.table1.#{__method__}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :weapon, true )
    end

    def self.e3( investigator )
      EEventLog.log( I18n.t( "events.table1.#{__method__}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :sign, true )
    end

    def self.e4( investigator )
      EEventLog.log( I18n.t( "events.table1.#{__method__}" ) )
      investigator.see_psy!
    end

    def self.e6( investigator )
      EEventLog.log( I18n.t( "events.table1.#{__method__}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :spell, true )
    end

    def self.e7( investigator )
      EEventLog.log( I18n.t( "events.table1.#{__method__}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :weapon, false )
    end

    def self.e11( investigator )
      EEventLog.log( I18n.t( "events.table1.#{__method__}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :weapon, true )
    end

    def self.e12( investigator )
      EEventLog.log( I18n.t( "events.table1.#{__method__}" ) )
      investigator.decrement( :san, 3 )
      investigator.update_attribute( :spell, true )
    end

    def self.method_missing( _, _ )
      EEventLog.log( 'event non implementé' )
    end

  end
end

