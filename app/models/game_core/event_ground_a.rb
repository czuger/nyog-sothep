module GameCore
  class EventGroundA

    def self.table1_e1( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.current_location = investigator.last_location
      investigator.save!
    end

    def self.table1_e2( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :weapon, true )
    end

    def self.table1_e3( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :sign, true )
    end

    def self.table1_e4( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.helped_by_kown_psy!
    end

    def self.table1_e5( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      PProfPosition.set_random_positions( game_board, professor.current_location, 3 )
    end

    def self.table1_e6( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :spell, true )
    end

    def self.table1_e7( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :weapon, false )
    end

    # def self.table1_e8( game_board, investigator, professor )
    #   EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
    #   investigator.car_break_down!
    # end

    def self.table1_e9( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.update_attribute( :medaillon, true )
      investigator.play_again!
    end

    def self.table1_e10( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      PProfPosition.set_random_positions( game_board, professor.current_location, 2 )
    end

    def self.table1_e11( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :weapon, true )
    end

    def self.table1_e12( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.decrement( :san, 3 )
      investigator.update_attribute( :spell, true )
    end

    def self.table1_e13( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      professor.update_attribute( :spotted, true )
    end

    def self.table1_e14( game_board, investigator, professor )
      EEventLog.log( game_board, ( I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) ) )
      investigator.play_again!
    end

    def self.method_missing( method_name, game_board, *args )
      raise "Method mising : #{method_name}" unless method_name =~ /table.*/
      EEventLog.log( game_board, 'event non implementé' )
    end

  end
end

