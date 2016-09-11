module GameLogic
  module EventGroundB

    def table2_e2( investigator )
      EEventLog.log( I18n.t( "events.table2.#{__method__.gsub('_','.')}" ) )
      investigator.see_psy!
    end

    def table2_e2( investigator )
      EEventLog.log( I18n.t( "events.table2.#{__method__.gsub('_','.')}" ) )
      investigator.decrement( :san, 3 )
      investigator.update_attribute( :weapon, true )
    end

    def table2_e3( investigator )
      EEventLog.log( I18n.t( "events.table2.#{__method__.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :spell, true )
    end

    def table2_e4( investigator )
      EEventLog.log( I18n.t( "events.table2.#{__method__.gsub('_','.')}" ) )
      investigator.play_again!
    end

    def table2_e5( investigator )
      EEventLog.log( I18n.t( "events.table2.#{__method__.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :sign, true )
    end

    def table2_e8( investigator )
      EEventLog.log( I18n.t( "events.table2.#{__method__.gsub('_','.')}" ) )
      investigator.decrement( :san, 3 )
      investigator.update_attribute( :medaillon, true )
    end

    def table2_method_missing( _, _ )
      EEventLog.log( 'event non implementé' )
    end

  end
end

