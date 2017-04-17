module GameCore
  module EventGroundA

    private

    # def table1_e1( game_board, professor )
    #   EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
    #   goes_back( game_board )
    # end

    def table1_e2( game_board, _ )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      update_attribute( :weapon, true ) if loose_san( game_board, 2 )
    end

    def table1_e3( game_board, _ )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      update_attribute( :sign, true ) if loose_san( game_board, 2 )
    end

    # def table1_e4( game_board, professor )
    #   EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
    #   # TODO : add san bonus on next turn, not implemented currently
    #   skip_next_turn!
    # end

    def table1_e5( game_board, _ )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      ask_prof_for_fake_cities( game_board, 2 )
    end

    def table1_e6( game_board, _ )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      update_attribute( :spell, true ) if loose_san( game_board, 2 )
    end

    def table1_e7( game_board, _ )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      update_attribute( :weapon, false ) if loose_san( game_board, 2 )
    end

    # def table1_e8( game_board, professor )
    #   EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
    #   car_break_down!
    # end

    # def table1_e9( game_board, professor )
    #   EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
    #   update_attribute( :medaillon, true )
    #   replay!
    # end

    def table1_e10( game_board, _ )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      ask_prof_for_fake_cities( game_board, 1 )
    end

    def table1_e11( game_board, _ )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      update_attribute( :weapon, true ) if loose_san( game_board, 2 )
    end

    def table1_e12( game_board, _ )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      update_attribute( :spell, true ) if loose_san( game_board, 3 )
    end

    def table1_e13( game_board, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      prof_spotted( game_board, professor )
    end

    # def table1_e14( game_board, _ )
    #   EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
    #   replay!
    # end

    def table1_e18( game_board, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      prof_spotted( game_board, professor )
    end

  end
end

