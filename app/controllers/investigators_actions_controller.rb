class InvestigatorsActionsController < ApplicationController

  include GameLogic::Dices
  include GameLogic::Movement
  include GameLogic::Events

  def special_event
    set_game_board
    set_current_investigator

    log_event_for_investigator( @current_investigator )
    case @current_investigator.aasm_state
      when 'known_psy_help' then
        @current_investigator.increment( :san, 5 )
        @current_investigator.finalize_event!
      when 'delayed' then
        @current_investigator.finalize_event!
      else
        raise "Unknown aasm_state : #{investigator.aasm_state}"
    end
    EEventLog.log( @game_board, I18n.t( 'actions.result.pass' ) )

    redirect_to map_show_url
  end


  def move
    set_game_board

    raise "Investigator move called while game_board not in inv_move state : #{@game_board.inspect}" unless @game_board.inv_move?

    if params['zone_id'] && params['zone_class']
      dest_loc = params['zone_class'].constantize.find( params['zone_id'] )
    else
      raise "Movement destinatin not set : #{params.inspect}"
    end

    ActiveRecord::Base.transaction do
      EEventLog.start_event_block( @game_board )
      set_current_investigator

      # TODO : need to fix special events issues.
      unless @current_investigator.ready_to_move?
        raise "Investigator able to move only if ready : #{@current_investigator.inspect}"
      end

      if move_current_investigator( dest_loc )
        @current_investigator.moved!
      else
        @current_investigator.didnt_move!
      end

      check_end_of_movements
    end

    redirect_to map_show_url
  end


  def go_psy
    set_game_board
    raise "Go psy called while game_board not in inv_move state : #{@game_board.inspect}" unless @game_board.inv_move?

    ActiveRecord::Base.transaction do

      EEventLog.start_event_block( @game_board )

      set_current_investigator

      if @current_investigator.current_location.city?
        san = d6
        @current_investigator.increment!( :san, san )
        EEventLog.log( @game_board, I18n.t( "actions.result.psy.#{@current_investigator.gender}",
          san: san, investigator_name: t( "investigators.#{@current_investigator.code_name}" ) ) )

        @current_investigator.didnt_move!

        check_end_of_movements
      else
        raise "Psy asked in non city area : #{params.inspect}"
      end
    end

    redirect_to map_show_url
  end

  private

  def check_end_of_movements
    # If all the investigators have moved, we set the game board on next state
    if @game_board.i_investigators.where( aasm_state: :ready_to_move ).count == 0
      @game_board.inv_move_end!
    end

  end

end
