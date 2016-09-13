module GameLogic::InvestigatorsTurn

  # TODO : manage game board events chain (prof -> investigators -> etc ...)
  # First access to the map mape prof play if prof_turn

  def set_current_moving_investigator
    @current_investigator = @game_board.i_investigators.where( aasm_state: [ :ready, :known_psy_help, :delayed, :replay ] ).order( :id ).first
  end

  # def set_next_investigator
  #
  #   unless @current_investigator.replay?
  #     begin
  #       cycle_investigators
  #
  #       # If investigator is delayed, we remove the delayed flag and then cycle again to the next investigator
  #       investigator_delayed = @current_investigator.delayed? || @current_investigator.great_psy_help?
  #       if investigator_delayed
  #         EEventLog.start_event_block( @game_board )
  #         EEventLog.log( @game_board, I18n.t( 'actions.result.pass', investigator_name: t( "investigators.#{@current_investigator.code_name}" ) ) )
  #         if @current_investigator.great_psy_help?
  #           EEventLog.log( @game_board, I18n.t( 'actions.result.psy_rep', investigator_name: t( "investigators.#{@current_investigator.code_name}" ) ) )
  #           @current_investigator.increment!( :san, 5 )
  #         end
  #         @current_investigator.reset!
  #       end
  #
  #     end while investigator_delayed
  #   else
  #     @current_investigator.reset!
  #   end
  #
  #   EEventLog.flush_old_events( @game_board )
  # end
  #
  # private
  #
  # def cycle_investigators
  #   next_investigator = @game_board.i_investigators.where( 'id > ?', @current_investigator.id ).order( :id ).first
  #   unless next_investigator
  #     # END OF THE TURN
  #
  #     GameCore::Professor.new( @game_board, @professor ).play
  #
  #     next_investigator = @game_board.i_investigators.order( :id ).first
  #   end
  #
  #   # Required in order to allow one investigator to play
  #   unless @current_investigator.id == next_investigator.id
  #     @current_investigator.update_attribute( :current, false )
  #     next_investigator.update_attribute( :current, true )
  #   end
  #   set_current_investigator
  # end

end