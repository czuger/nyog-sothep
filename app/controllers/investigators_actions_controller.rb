class InvestigatorsActionsController < ApplicationController

  include GameLogic::InvestigatorsTurn
  include GameLogic::Dices
  include GameLogic::Movement
  include GameLogic::Events

  def move
    if params['zone_id'] && params['zone_class']
      dest_loc = params['zone_class'].constantize.find( params['zone_id'] )
    else
      raise "Zone error : #{params.inspect}"
    end

    @game_board = GGameBoard.find( params[:g_game_board_id])
    raise "Game board not in inv_move state : #{@game_board.inspect}" unless @game_board.inv_move?

    set_current_moving_investigator

    ActiveRecord::Base.transaction do
      EEventLog.start_event_block( @game_board )
      if @current_investigator
        move_current_investigator( dest_loc )
        @current_investigator.move!

        set_current_moving_investigator
        # If all the investigators have moved, we roll the events
        unless @current_investigator
          @game_board.inv_move_end!
          @professor = @game_board.p_professor
          @game_board.i_investigators.where( aasm_state: :move_done ).order( :id ).each do |investigator|
            roll_event( investigator )
          end
          EEventLog.flush_old_events( @game_board )
          # After that, the game board goes to next step
          @game_board.inv_event_end!
          @game_board.inv_fight_prof_end!
        end
      end
    end

    redirect_to investigators_map_show_url
  end


  def go_psy

    @game_board = GGameBoard.find( params[:g_game_board_id])

    ActiveRecord::Base.transaction do

      EEventLog.start_event_block( @game_board )

      set_current_moving_investigator

      if @current_investigator.current_location.city?
        san = d6
        @current_investigator.increment!( :san, san )
        EEventLog.log( @game_board, I18n.t( "actions.result.psy.#{@current_investigator.gender}",
          san: san, investigator_name: t( "investigators.#{@current_investigator.code_name}" ) ) )

        @current_investigator.move
      else
        raise "Psy asked in non city area : #{params.inspect}"
      end

    end

    redirect_to investigators_map_show_url
  end

end
