module GameLogic::Movement

  def move_current_investigator( dest_loc )

    current_loc = @current_investigator.current_location

    unless cross_border_forbidden( current_loc, dest_loc )
      @current_investigator.current_location = dest_loc
      event = "#{@current_investigator.code_name.humanize} se déplace à #{dest_loc.code_name.humanize}"
      EEventLog.create!( event: event )
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
        if dice <= 2
          event = "#{@current_investigator.code_name.humanize} a été empêché de passer la frontière par une patrouille."
          EEventLog.create!( event: event )
          border_forbidden = true
        end
      end
    end

    border_forbidden
  end

end