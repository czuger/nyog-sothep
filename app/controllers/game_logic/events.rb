module GameLogic::Events

  private

  def process_events
    raise "Process events called while game_board not in inv_event state : #{@game_board.inspect}" unless @game_board.inv_event?
    if @game_board.next_moving_investigator
      raise "No investigators should be ready on event phase : #{@game_board.next_moving_investigator.inspect}"
    end

    # pp @game_board.i_investigators

    @professor = @game_board.p_professor

    ActiveRecord::Base.transaction do
      @professor.update( spotted: false )
      @game_board.p_prof_positions.delete_all

      begin
        next_investigator_for_event = @game_board.next_investigator_ready_for_event
        # puts "Next investigator ready for event = #{next_investigator_for_event.inspect}"
        if next_investigator_for_event
          EEventLog.log_event_for_investigator( @game_board, next_investigator_for_event )

          @game_board.resolve_encounter( next_investigator_for_event )

          if next_investigator_for_event.roll_event?
            GameCore::Events.roll_event( @game_board, next_investigator_for_event, @professor )
          else
            EEventLog.log( @game_board, I18n.t( 'log.pass' ) )
          end

          unless next_investigator_for_event.dead
            # If nothing else happens, then the investigator is ready for next adventures
            next_investigator_for_event.play_next_turn! if next_investigator_for_event.roll_event?
            next_investigator_for_event.play_next_turn_after_passing_turn! if next_investigator_for_event.roll_no_event?

            # If we have someone that replay : then break the loop
            replay = next_investigator_for_event.replay?
            break if replay
          else
            next_investigator_for_event.destroy!
          end

        end

      end while next_investigator_for_event

      if replay
        next_investigator_for_event.inv_move_after_replay!
        @game_board.someplayer_shoot_again!
      else
        @game_board.prof_move!
      end

      @loop = true

      EEventLog.flush_old_events( @game_board )
    end
  end
end