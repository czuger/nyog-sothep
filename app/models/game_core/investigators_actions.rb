module GameCore
  class InvestigatorsActions

    def initialize( game_board, prof )
      @game_board = game_board
      @prof = prof
    end

    def investigators_ia_play
      ActiveRecord::Base.transaction do

        case @game_board.aasm_state
          when 'inv_move'
            investigator_move
          when 'inv_events'
            investigator_events
            # Break out of the while
          else
            raise "Bad aasm_state : #{@game_board.aasm_state}"
        end

        # Once all investigators have played, we goes to next turn
        unless @game_board.prof_asked_for_fake_cities? || some_investigator_is_ready_to_play?
          @game_board.next_turn
        end

      end
    end

    private

    def investigator_events
      pp @game_board.investigators_in_misty_things.reload

      # Unlock delayed investigators
      @game_board.investigators_in_misty_things.each do |i|
        i.exit_misty_things!
      end

      # Events for investigators
      @game_board.ready_for_events_investigators.each do |i|

        @game_board.resolve_encounter( i )

        i.ia_play_events( @game_board, @prof )
        # Break out of the investigators loop
        break if @game_board.prof_asked_for_fake_cities?

        # When the turn of the investigator is finished we need to check for a prof fight
        i.check_for_prof_to_fight_in_city( @game_board, @prof )

        # If investigator is still alive
        i.events_done! if i.events?
      end
    end

    def investigator_move
      @game_board.ready_to_move_investigators.each do |i|
        i.ia_play_movements( @game_board, @prof )
      end
      @game_board.inv_movement_done!
    end

    def some_investigator_is_ready_to_play?
      @game_board.ready_to_move_investigators.count + @game_board.ready_for_events_investigators.count > 0
    end

  end
end

