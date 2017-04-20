module GameCore::Movement

  private

  def regular_move_token( gb, token, dest_loc )

    assert_regular_movement_allowed( token.current_location, dest_loc )

    # p token

    if token.class == PProfessor || cross_border_allowed( gb, token, dest_loc )

      token.last_location = token.current_location unless token.class == PProfessor

      token.current_location = dest_loc
      token.save!

      unless token.class == PProfessor
        EEventLog.log_investigator_movement( gb, token, dest_loc )
      end

      return true
    end

    false
  end

  def cross_border_allowed( gb, token, dest_loc )

    border_allowed = true

    if dest_loc.city? && token.current_location.city?
      road = token.current_location.outgoing_r_roads.where( dest_city_id: dest_loc.id )
      raise "More than one road : #{road.inspect}" if road.count > 1

      road = road.first

      if road.border
        dice = GameCore::Dices.d6
        if dice >= 5
          inv_name = I18n.t( "investigators.#{token.code_name}" )
          event = I18n.t( "border_control.#{token.gender}", investigator_name: inv_name )
          EEventLog.log( gb, self, event )
          border_allowed = false
        end
      end
    end

    border_allowed
  end


  def assert_regular_movement_allowed( src, dest )
    unless src.destinations.include?( dest )
      raise "Illegal move attempt : token.current_location = #{src.inspect}, dest = #{dest.inspect}"
    end
  end

end