class EEventLog < ApplicationRecord

  def self.flush_old_events
    e = EEventLog.all.order( :id ).pluck( :id )
    keeping_e = e.last( 20 )
    EEventLog.where( id: e - keeping_e ).delete_all unless ( e - keeping_e ).empty?
  end

end
