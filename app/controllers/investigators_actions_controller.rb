class InvestigatorsActionsController < ApplicationController

  include GameLogic::GameBoardStatusRedirection

  def investigators_ia_play

    set_game_board

    ActiveRecord::Base.transaction do

      while some_investigator_is_ready_to_play?

        case @game_board.aasm_state
          when 'inv_move'
            # Investigators move
            @game_board.ready_to_move_investigators.each do |i|
              i.ia_play_movements( @game_board, @prof )
            end
            @game_board.inv_movement_done!
          when 'inv_events'
            # Events for investigators
            @game_board.ready_for_events_investigators.each do |i|
              i.ia_play_events( @game_board, @prof )
              break if @game_board.prof_asked_for_fake_cities?
            end

            if @game_board.ready_for_events_investigators.count == 0
              # if no more investigators ready to play, then we start a new turn
              if @game_board.ready_to_move_investigators.count == 0 &&
                @game_board.alive_investigators.each do |i|
                  i.update( current: true )
                end

                # Prof positions are forgotten over time
                IInvTargetPosition.where( g_game_board_id: @game_board.id ).update_all( 'memory_counter = memory_counter - 1' )
                IInvTargetPosition.where( g_game_board_id: @game_board.id ).delete_all( 'memory_counter <= 0' )

                @game_board.inv_movement_done!
                @game_board.next_turn
              end
            end

          else
            raise "Bad aasm_state : #{@game_board.aasm_state}"
        end
      end
    end

    check_prof_asked_for_fake_cities{ redirect_to g_game_board_play_url( g_game_board_id: @game_board.id ) }

  end

  private

  def some_investigator_is_ready_to_play?
    @game_board.ready_to_move_investigators.count + @game_board.ready_for_events_investigators.count > 0
  end

end
