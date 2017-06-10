module GameCore::Movement

  private

  def regular_move_token( gb, token, dest_loc )

    assert_regular_movement_allowed( token.current_location, dest_loc )

    # p token

    if token.class == PProfessor || cross_border_allowed( gb, token, dest_loc )

      token.last_location_code_name = token.current_location_code_name unless token.class == PProfessor

      token.current_location_code_name = dest_loc.code_name
      token.save!

      unless token.class == PProfessor
        EEventLog.log_investigator_movement( gb, token, dest_loc.code_name )
      end

      return true
    end

    false
  end

  def cross_border_allowed( gb, token, dest_loc )

    border_allowed = true

    if dest_loc.city? && token.current_location.city?
      if GameCore::Map::BordersCrossings.check?( token.current_location_code_name, dest_loc.code_name )
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
    unless src.destinations_codes_names.include?( dest.code_name )
      raise "Illegal move attempt : #{dest.code_name.inspect} not in #{src.destinations_codes_names}"
    end
  end

end