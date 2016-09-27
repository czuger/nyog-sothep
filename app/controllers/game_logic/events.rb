module GameLogic::Events

  include GameLogic::Dices

  def process_events
    raise "Process events called while game_board not in inv_event state : #{@game_board.inspect}" unless @game_board.inv_event?
    if @game_board.next_moving_investigator
      raise "No investigators should be ready on event phase : #{@game_board.next_moving_investigator.inspect}"
    end

    # pp @game_board.i_investigators

    @professor = @game_board.p_professor
    replay = false

    ActiveRecord::Base.transaction do
      @professor.update( spotted: false )
      @game_board.p_prof_positions.delete_all

      begin
        next_investigator_for_event = @game_board.next_investigator_ready_for_event
        # puts "Next investigator ready for event = #{next_investigator_for_event.inspect}"
        if next_investigator_for_event
          log_event_for_investigator( next_investigator_for_event )

          # Il faut gérer les rencontres
          monster = @game_board.p_monsters_positions.find_by_location_id( next_investigator_for_event.location_id )
          if monster
            if next_investigator_for_event.location_id ==
          end

          if next_investigator_for_event.roll_event?
            roll_event( next_investigator_for_event )
          else
            EEventLog.log( @game_board, I18n.t( 'log.pass' ) )
          end

          # If nothing else happens, then the investigator is ready for next adventures
          next_investigator_for_event.play_next_turn! if next_investigator_for_event.roll_event?
          next_investigator_for_event.play_next_turn_after_passing_turn! if next_investigator_for_event.roll_no_event?

          # If we have someone that replay : then break the loop
          replay = next_investigator_for_event.replay?
          break if replay

        end

      end while next_investigator_for_event

      if replay
        next_investigator_for_event.inv_move_after_replay!
        @game_board.someplayer_shoot_again!
      else
        @game_board.inv_event_end!
      end

      EEventLog.flush_old_events( @game_board )
    end
    replay
  end

  private

  def log_event_for_investigator( investigator )
    EEventLog.start_event_block( @game_board )
    iname = I18n.t( "investigators.#{investigator.code_name}" )
    EEventLog.log( @game_board, I18n.t( 'log.incarn', investigator_name: iname ) )
  end

  def roll_event( investigator )
    if investigator.current_location.city? && investigator.last_location.city?

      roll = event_dices( investigator )

      if investigator.event_table == 1
        GameCore::EventGroundA.send( "table#{investigator.event_table}_e#{roll}", @game_board, investigator, @professor )
      elsif investigator.event_table == 2
        GameCore::EventGroundB.send( "table#{investigator.event_table}_e#{roll}", @game_board, investigator, @professor )
      else
        raise "Bad event table for investigator : #{investigator.inspect}"
      end
    else
      EEventLog.log( @game_board, I18n.t( 'events.no_water_events' ) )
    end
  end

  def event_dices( investigator )
    roll = d6
    roll += d6 if investigator.sign
    roll += d6 if investigator.sign && investigator.medaillon
    roll
  end

end