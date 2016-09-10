module GameLogic::Movement

  def move_current_investigator( dest_loc )

    current_loc = @current_investigator.current_location

    unless cross_border_forbidden( current_loc, dest_loc )
      @current_investigator.current_location = dest_loc

      translated_dest_loc = I18n.t( "locations.#{dest_loc.code_name}", :default => "à #{dest_loc.code_name.humanize}" )
      event = "#{I18n.t( "investigators.#{@current_investigator.code_name}" )} se déplace #{translated_dest_loc}"
      EEventLog.log( event )
      return true
    end

    false
  end

  private

  def cross_border_forbidden( current_loc, dest_loc )

    border_forbidden = false

    if dest_loc.class == CCity && current_loc.class == CCity
      road = current_loc.outgoing_r_roads.where( dest_city_id: dest_loc.id )
      raise "More than one road : #{road.inspect}" if road.count > 1

      road = road.first
      # puts "Src city : #{road.src_city.inspect}"
      # puts "Dest city : #{road.dest_city.inspect}"
      # puts "Road : #{road.inspect}"

      if road.border
        dice = rand( 1 .. 6 )
        if dice >= 5
          inv_name = I18n.t( "investigators.#{@current_investigator.code_name}" )
          event = I18n.t( "border_control.#{@current_investigator.gender}", investigator_name: inv_name )
          EEventLog.log( event )
          border_forbidden = true
        end
      end
    end

    border_forbidden
  end

end