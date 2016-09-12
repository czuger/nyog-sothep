class ActionsController < ApplicationController

  include GameLogic::Turn
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

    ActiveRecord::Base.transaction do

      EEventLog.start_event_block( @game_board )

      set_current_investigator
      if move_current_investigator( dest_loc )
        roll_event
      end
      set_next_investigator

    end

    redirect_to g_game_board_maps_url
  end


  def go_psy

    @game_board = GGameBoard.find( params[:g_game_board_id])

    ActiveRecord::Base.transaction do

      EEventLog.start_event_block( @game_board )

      set_current_investigator

      if @current_investigator.current_location.city?
        @current_investigator.pass_next_turn
        san = d6
        @current_investigator.increment!( :san, san )
        EEventLog.log( @game_board, I18n.t( "actions.result.psy.#{@current_investigator.gender}",
          san: san, investigator_name: t( "investigators.#{@current_investigator.code_name}" ) ) )

        set_next_investigator
      else
        raise "Psy asked in non city area : #{params.inspect}"
      end

    end

    redirect_to g_game_board_maps_url
  end

end
