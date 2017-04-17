class InvestigatorsActionsController < ApplicationController

  include GameLogic::GameBoardStatusRedirection

  def investigators_ia_play

    set_game_board

    ActiveRecord::Base.transaction do

      # New turn here
      # if no more investigators ready to play, then we start a new turn
      if @game_board.ready_to_play_investigators.count == 0
        @game_board.alive_investigators.each do |i|
          i.update( current: true )
        end

        # Prof positions are forgotten over time
        IInvTargetPosition.where( g_game_board_id: @game_board.id ).update_all( 'memory_counter = memory_counter - 1' )
        IInvTargetPosition.where( g_game_board_id: @game_board.id ).delete_all( 'memory_counter <= 0' )

        @game_board.next_turn
      end

      # Investigators play
      @game_board.ready_to_play_investigators.each do |i|
        i.ia_play( @game_board, @prof )
        break if @game_board.prof_asked_for_fake_cities?
      end

    end

    check_prof_asked_for_fake_cities{ redirect_to g_game_board_play_url( g_game_board_id: @game_board.id ) }

  end

end
