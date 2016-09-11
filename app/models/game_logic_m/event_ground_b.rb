module GameLogicM
  class EventGroundB

    def self.e2( investigator )
      EEventLog.log( I18n.t( "events.table2.#{__method__}" ) )
      investigator.see_psy!
    end

    def self.e2( investigator )
      EEventLog.log( I18n.t( "events.table2.#{__method__}" ) )
      investigator.decrement( :san, 3 )
      investigator.update_attribute( :weapon, true )
    end

    def self.e3( investigator )
      EEventLog.log( I18n.t( "events.table2.#{__method__}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :spell, true )
    end

    def self.e4( investigator )
      EEventLog.log( I18n.t( "events.table2.#{__method__}" ) )
      investigator.play_again!
    end

    def self.e5( investigator )
      EEventLog.log( I18n.t( "events.table2.#{__method__}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :sign, true )
    end

    def self.e8( investigator )
      EEventLog.log( I18n.t( "events.table2.#{__method__}" ) )
      investigator.decrement( :san, 3 )
      investigator.update_attribute( :medaillon, true )
    end

    def self.method_missing( _, _ )
      EEventLog.log( 'event non implementé' )
    end

  end
end

