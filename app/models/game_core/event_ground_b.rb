module GameCore
  class EventGroundB

#     def self.table2_e1( game_board, investigator, professor )
#       EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
# #       investigator.helped_by_kown_psy!
#     end

    def self.table2_e2( game_board, investigator, professor )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 3 )
      investigator.update_attribute( :weapon, true )
    end

    def self.table2_e3( game_board, investigator, professor )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :spell, true )
    end

    def self.table2_e4( game_board, investigator, professor )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.replay!
    end

    def self.table2_e5( game_board, investigator, professor )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :sign, true )
    end

    def self.table2_e6( game_board, investigator, professor )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      PProfPosition.set_random_positions( game_board, professor.current_location, 3 )
    end

    def self.table2_e7( game_board, investigator, professor )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.current_location = investigator.last_location
      investigator.save!
    end

    def self.table2_e8( game_board, investigator, professor )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 3 )
      investigator.update_attribute( :medaillon, true )
    end

    def self.table2_e9( game_board, investigator, professor )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      PProfPosition.set_random_positions( game_board, professor.current_location, 2 )
    end

    def self.table2_e10( game_board, investigator, professor )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :spell, true )
      investigator.replay!
    end

    def self.table2_e11( game_board, investigator, professor )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.update_attribute( :weapon, true )
      investigator.replay!
    end

    def self.table2_e12( game_board, investigator, professor )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
    end

    def self.table2_e17( game_board, investigator, professor )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.replay!
    end

    def self.table2_e18( game_board, investigator, professor )
      EEventLog.log( game_board, I18n.t( "events.#{__method__.to_s.gsub('_','.')}" ) )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :weapon, false )
    end

    def self.method_missing( method_name, game_board, *args )
      raise "Method mising : #{method_name}" unless method_name =~ /table.*/
      EEventLog.log( game_board, 'event non implementé' )
    end

  end
end

