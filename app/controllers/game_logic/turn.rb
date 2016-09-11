module GameLogic::Turn

  def set_current_investigator
    @current_investigator = IInvestigator.find_by( current: true )
  end

  def set_next_investigator

    unless @current_investigator.replay?
      begin
        cycle_investigators

        # If investigator is delayed, we remove the delayed flag and then cycle again to the next investigator
        investigator_delayed = @current_investigator.delayed? || @current_investigator.delayed_and_increase_san?
        if investigator_delayed
          EEventLog.start_event_block
          EEventLog.log( I18n.t( 'actions.result.pass', investigator_name: t( "investigators.#{@current_investigator.code_name}" ) ) )
          if @current_investigator.delayed_and_increase_san?
            EEventLog.log( I18n.t( 'actions.result.psy_rep', investigator_name: t( "investigators.#{@current_investigator.code_name}" ) ) )
            @current_investigator.increment!( :san, 5 )
          end
          @current_investigator.reset!
        end

      end while investigator_delayed
    else
      @current_investigator.reset!
    end

    EEventLog.flush_old_events
  end

  private

  def cycle_investigators
    next_investigator = IInvestigator.where( 'id > ?', @current_investigator.id ).order( :id ).first
    unless next_investigator

      # Move professor.
      prof = PProfessor.first
      loc = prof.current_location
      dests = []
      if loc.class == CCity
        dests += loc.dest_cities
        dests << loc.w_water_area
      else
        dests += loc.ports
        dests += loc.connected_w_water_areas
      end
      puts dests.inspect

      prof.current_location = dests.compact.sample
      prof.save!
      # EEventLog.log( "Prof se déplace : #{prof.current_location.inspect}" )

      next_investigator = IInvestigator.order( :id ).first unless next_investigator
    end

    @current_investigator.update_attribute( :current, false )
    next_investigator.update_attribute( :current, true )
    set_current_investigator
  end

end