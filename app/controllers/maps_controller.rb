require 'pp'

class MapsController < ApplicationController

  include GameLogic::Movement
  include GameLogic::Turn
  include GameLogic::Events

  def show

    @current_investigator = IInvestigator.find_by( current: true )
    @zone = @current_investigator.current_location

    @events = EEventLog.all.order( 'id DESC' )

    @aval_destinations = []
    if @zone.class == CCity
      @aval_destinations += @zone.dest_cities
      @aval_destinations << @zone.w_water_area if @zone.w_water_area
    elsif @zone.class == WWaterArea
      @aval_destinations += @zone.ports
      @aval_destinations += @zone.connected_w_water_areas
    end

  end

  def update
    if params['zone_id'] && params['zone_class']
      dest_loc = params['zone_class'].constantize.find( params['zone_id'] )
    else
      raise "Zone error : #{params.inspect}"
    end

    ActiveRecord::Base.transaction do

      set_current_investigator
      if move_current_investigator( dest_loc )
        roll_event
      end
      set_next_investigator

      EEventLog.flush_old_events
    end

    redirect_to maps_url
  end
end
