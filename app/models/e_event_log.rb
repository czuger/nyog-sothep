class EEventLog < ApplicationRecord

  def self.flush_old_events
    e = EEventLog.all.order( :id ).pluck( :id )
    keeping_e = e.last( 30 )
    EEventLog.where( id: e - keeping_e ).delete_all unless ( e - keeping_e ).empty?
  end

  def self.start_event_block
    ActiveRecord::Base.connection.execute("SELECT nextval('e_event_logs_logset')")
    EEventLog.log( '*' )
  end

  def self.log( event )
    logset = ActiveRecord::Base.connection.select_value("SELECT currval('e_event_logs_logset')")
    self.create!( logset: logset, event: event )
  end

end
