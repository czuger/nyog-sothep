module GameLogic::Events

  include GameLogic::Dices

  def roll_events
    set_game_board
    raise "Roll events called while game_board not in inv_event state : #{@game_board.inspect}" unless @game_board.inv_event?
    if investigators_ready_to_move.count > 0
      raise "No investigators should be ready on event phase : #{investigators_ready_to_move.inspect}"
    end

    @professor = @game_board.p_professor

    ActiveRecord::Base.transaction do
      @professor.update( spotted: false )
      @game_board.p_prof_positions.delete_all

      @game_board.i_investigators.where( aasm_state: :move_phase_done ).order( :id ).each do |investigator|
        log_event_for_investigator( investigator )
        roll_event( investigator )

        replay_inv = investigators_ready_to_move
        if replay_inv.count > 0
          @game_board.players_replay!
          break
        end
      end
      EEventLog.flush_old_events( @game_board )

      replay_inv = investigators_ready_to_move
      if replay_inv.count == 0
        @game_board.inv_event_end!
      end
    end

    redirect_to map_show_url
  end

  private

  def log_event_for_investigator( investigator )
    EEventLog.start_event_block( @game_board )
    iname = I18n.t( "investigators.#{investigator.code_name}" )
    EEventLog.log( @game_board, I18n.t( 'log.incarn', investigator_name: iname ) )
  end

  def roll_event( investigator )
    if investigator.current_location.city? && investigator.last_location.city?

      roll = d6
      roll += d6 if investigator.sign
      roll += d6 if investigator.sign && investigator.medaillon

      if investigator.event_table == 1
        GameCore::EventGroundA.send( "table#{investigator.event_table}_e#{roll}", @game_board, investigator, @professor )
      elsif investigator.event_table == 2
        GameCore::EventGroundA.send( "table#{investigator.event_table}_e#{roll}", @game_board, investigator, @professor )
      end
    end
    # If nothing else happens, then the investigator is ready for next adventures
    investigator.finalize_event! if investigator.move_phase_done?
  end

end