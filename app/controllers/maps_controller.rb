require 'pp'

class MapsController < ApplicationController
  def show

    if params['prof_zone_id'] && params['prof_zone_class']
      @prof_zone = params['prof_zone_class'].constantize.find( params['prof_zone_id'] )
    else
      @prof_zone = CCity.find_by( code_name: :innsmouth )
    end
    @prof_destinations = []
    if @prof_zone.class == CCity
      @prof_destinations += @prof_zone.dest_cities
      @prof_destinations << @prof_zone.w_water_area if @prof_zone.w_water_area
    elsif @prof_zone.class == WWaterArea
      @prof_destinations += @prof_zone.ports
      @prof_destinations += @prof_zone.connected_w_water_areas
    end
  end

  def update
    redirect_to maps_url
  end
end
