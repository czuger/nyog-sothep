module GameCore
  module EventGroundA

    private

    def table1_e1( game_board, _ )
      EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
      goes_back( game_board )
    end

    def table1_e2( game_board, _ )
      EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
      update_attribute( :weapon, true ) if loose_san( game_board,  2 )
    end

    def table1_e3( game_board, _ )
      EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
      update_attribute( :sign, true ) if loose_san( game_board,  2 )
    end

    def table1_e4( game_board, _ )
      EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
      loose_turn_and_gain_san( 1, 5 )
    end

    def table1_e5( game_board, _ )
      EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
      ask_prof_for_fake_cities( game_board, 2 ) unless game_board.p_professor.current_location.kind_of?( WWaterArea )
    end

    def table1_e6( game_board, _ )
      EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
      update_attribute( :spell, true ) if loose_san( game_board,  2 )
    end

    def table1_e7( game_board, _ )
      EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
      update_attribute( :weapon, false ) if loose_san( game_board,  2 )
    end

    # def table1_e8( game_board, self, professor )
    #   EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
    #   car_break_down!
    # end

    # def table1_e9( game_board, self, professor )
    #   EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
    #   update_attribute( :medaillon, true )
    #   replay!
    # end

    def table1_e10( game_board, _ )
      EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
      ask_prof_for_fake_cities( game_board, 1 ) unless game_board.p_professor.current_location.kind_of?( WWaterArea )
    end

    def table1_e11( game_board, _ )
      EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
      update_attribute( :weapon, true ) if loose_san( game_board,  2 )
    end

    def table1_e12( game_board, _ )
      EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
      update_attribute( :spell, true ) if loose_san( game_board,  3 )
    end

    def table1_e13( game_board, professor )
      EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
      professor.spotted( game_board )
    end

    # def table1_e14( game_board, _ )
    #   EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
    #   replay!
    # end

    def table1_e18( game_board, professor )
      EEventLog.log( game_board, self, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}.#{gender}", investigator_name: translated_name ) ) )
      professor.spotted( game_board )
    end

  end
end

