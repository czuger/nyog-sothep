module GameCore
  module EventGroundB

    private

#     def table2_e1( game_board, professor )
#       EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
# #       helped_by_kown_psy!
#     end

    def table2_e2( game_board, _ )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      update_attribute( :weapon, true ) if loose_san( game_board, 3 )
    end

    def table2_e3( game_board, _ )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      update_attribute( :spell, true ) if loose_san( game_board, 2 )
    end

    # def table2_e4( game_board, professor )
    #   EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
    #   replay!
    # end

    def table2_e5( game_board, _ )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      update_attribute( :sign, true ) if loose_san( game_board, 2 )
    end

    def table2_e6( game_board, _ )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      ask_prof_for_fake_cities( game_board, 2 )
    end

    # def table2_e7( game_board, professor )
    #   EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
    #   goes_back( game_board )
    # end

    def table2_e8( game_board, _ )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      update_attribute( :medaillon, true ) if loose_san( game_board, 2 )
    end

    def table2_e9( game_board, _ )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      ask_prof_for_fake_cities( game_board, 1 )
    end

    # def table2_e10( game_board, professor )
    #   EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
    #   if loose_san( game_board, 2 )
    #     update_attribute( :spell, true )
    #     replay!
    #   end
    # end

    # def table2_e11( game_board, professor )
    #   EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
    #   update_attribute( :weapon, true )
    #   replay!
    # end

    def table2_e12( game_board, _ )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      loose_san( game_board, 2 )
    end

    def table2_e13( game_board, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      prof_spotted( game_board, professor )
    end

    # def table2_e17( game_board, professor )
    #   EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
    #   replay!
    # end

    def table2_e18( game_board, _ )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      update_attribute( :weapon, false ) if loose_san( game_board, 2 )
    end

  end
end

