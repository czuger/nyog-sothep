module GameCore
  class EventGroundA

    extend GameCore::CommonMethods

    def self.table1_e1( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator_goes_back( game_board, investigator )
    end

    def self.table1_e2( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.update_attribute( :weapon, true ) if investigator_loose_san( game_board, investigator, 2 )
    end

    def self.table1_e3( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.update_attribute( :sign, true ) if investigator_loose_san( game_board, investigator, 2 )
    end

    # def self.table1_e4( game_board, investigator, professor )
    #   EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
    #   # TODO : add san bonus on next turn, not implemented currently
    #   investigator.skip_next_turn!
    # end

    def self.table1_e5( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      PProfPosition.set_random_positions( game_board, professor.current_location, 3 )
    end

    def self.table1_e6( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.update_attribute( :spell, true ) if investigator_loose_san( game_board, investigator, 2 )
    end

    def self.table1_e7( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.update_attribute( :weapon, false ) if investigator_loose_san( game_board, investigator, 2 )
    end

    # def self.table1_e8( game_board, investigator, professor )
    #   EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
    #   investigator.car_break_down!
    # end

    def self.table1_e9( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.update_attribute( :medaillon, true )
      investigator.replay!
    end

    def self.table1_e10( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      PProfPosition.set_random_positions( game_board, professor.current_location, 2 )
    end

    def self.table1_e11( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.update_attribute( :weapon, true ) if investigator_loose_san( game_board, investigator, 2 )
    end

    def self.table1_e12( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.update_attribute( :spell, true ) if investigator_loose_san( game_board, investigator, 3 )
    end

    def self.table1_e13( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      professor.update_attribute( :spotted, true )
    end

    def self.table1_e14( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.replay!
    end

    def self.method_missing( method_name, game_board, *args )
      raise "Method mising : #{method_name}" unless method_name =~ /table.*/
      EEventLog.log( game_board, 'event non implementé' )
    end

  end
end

