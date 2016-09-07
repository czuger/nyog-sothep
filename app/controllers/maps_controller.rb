require 'pp'

class MapsController < ApplicationController

  include GameLogic::BorderCrossing

  def show

    @pos = PPosition.find_by( current: true )
    @zone = @pos.l_location

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
      zone = params['zone_class'].constantize.find( params['zone_id'] )
    else
      raise "Zone error : #{params.inspect}"
    end

    pos = PPosition.find_by( current: true )
    current_loc = pos.l_location
    pos.l_location = dest_loc = zone

    # Select next pos
    next_pos = PPosition.where( 'id > ?', pos.id ).order( :id ).first
    next_pos = PPosition.order( :id ).first unless next_pos

    ActiveRecord::Base.transaction do

      pos.update_attribute( :current, false )
      next_pos.update_attribute( :current, true )

      # Create events
      event = "#{pos.code_name.humanize} move to #{zone.code_name.humanize}"
      EEventLog.create!( event: event )

      check_cross_border( current_loc, dest_loc, pos )

      EEventLog.flush_old_events
    end

    redirect_to maps_url
  end
end
