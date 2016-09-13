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

end
