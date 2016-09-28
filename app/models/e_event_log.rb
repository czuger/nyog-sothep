class EEventLog < ApplicationRecord

  def self.flush_old_events( game_board )
    e = game_board.e_event_logs.all.order( :id ).pluck( :id )
    keeping_e = e.last( 30 )
    game_board.e_event_logs.where( id: e - keeping_e ).delete_all unless ( e - keeping_e ).empty?
  end

  def self.start_event_block( game_board )
    ActiveRecord::Base.connection.execute("SELECT nextval('e_event_logs_logset')")
    log( game_board, '*' )
  end

  def self.log( game_board, event )
    logset = ActiveRecord::Base.connection.select_value("SELECT currval('e_event_logs_logset')")
    game_board.e_event_logs.create!( logset: logset, event: event )
  end

  def self.log_investigator_movement( game_board, investigator, dest_loc, direction: :goes )

    translated_dest_loc = I18n.t( "locations.#{dest_loc.code_name}", :default => "à #{dest_loc.code_name.humanize}" )
    investigator_name = I18n.t( "investigators.#{investigator.code_name}" )

    event = case direction
      when :goes
        I18n.t( 'movement.goes', who: investigator_name, where: translated_dest_loc )
      when :return
        I18n.t( 'movement.return', who: investigator_name, where: translated_dest_loc )
      else
        raise "Unknown direction : #{direction}. Can only be :goes or :return"
    end

    EEventLog.log( game_board, event )

  end
end

