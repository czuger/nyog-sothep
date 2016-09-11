require 'pp'

class MapsController < ApplicationController

  include GameLogic::Movement
  include GameLogic::Turn
  include GameLogic::Events

  def switch_table
    params[:event_table]
    set_current_investigator
    @current_investigator.update_attribute( :event_table, params[:event_table] )
    head :ok
  end

  def show

    set_current_investigator
    @zone = @current_investigator.current_location

    @events = EEventLog.all.order( 'logset DESC, id ASC' )

    @prof = PProfessor.first
    @prof_zone = @prof.current_location

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

      EEventLog.start_event_block

      set_current_investigator
      if move_current_investigator( dest_loc )
        roll_event
      end
      set_next_investigator

    end

    redirect_to maps_url
  end
end
