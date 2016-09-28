module GameLogic::Movement

  include GameLogic::Dices

  def move_current_investigator( dest_loc )

    unless cross_border_forbidden( dest_loc )
      @current_investigator.last_location = @current_investigator.current_location
      @current_investigator.current_location = dest_loc
      @current_investigator.save!

      EEventLog.log_investigator_movement( @game_board, @current_investigator, dest_loc )

      return true
    end

    false
  end

  private

  def cross_border_forbidden( dest_loc )

    border_forbidden = false

    if dest_loc.city? && @current_investigator.current_location.city?
      road = @current_investigator.current_location.outgoing_r_roads.where( dest_city_id: dest_loc.id )
      raise "More than one road : #{road.inspect}" if road.count > 1

      road = road.first
      # puts "Src city : #{road.src_city.inspect}"
      # puts "Dest city : #{road.dest_city.inspect}"
      # puts "Road : #{road.inspect}"

      if road.border
        dice = d6
        if dice >= 5
          inv_name = I18n.t( "investigators.#{@current_investigator.code_name}" )
          event = I18n.t( "border_control.#{@current_investigator.gender}", investigator_name: inv_name )
          EEventLog.log( @game_board, event )
          border_forbidden = true
        end
      end
    end

    border_forbidden
  end

end