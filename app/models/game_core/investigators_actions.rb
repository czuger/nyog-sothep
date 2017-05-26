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
            # The regular case, we move then we play events
            skip_investigators_turn

            investigators_move
            investigators_actions

            # This is where we go after a break (user asked for options)
          when 'inv_events'
            skip_investigators_turn

            investigators_actions

          else
            raise "Bad aasm_state : #{@game_board.aasm_state}"
        end

        # Once all investigators have played, we goes to next turn
        # TODO : is this working with misty_step caus based on events ... maybe could lead to infinite loop
        unless @game_board.prof_asked_for_fake_cities? || some_investigator_is_ready_to_play?
          @game_board.next_turn
        end

      end
    end

    private

    def skip_investigators_turn
      @game_board.skip_turns_investigators.reload.each do |i|
        print "Investigator state : #{i.code_name} : #{i.aasm_state}"
        i.decrement!( :skip_turns )
        i.gain_san( @game_board, i.san_gain_after_lost_turns ) if i.san_gain_after_lost_turns && i.skip_turns <= 0
        i.skip_turn!
      end
    end

    def investigators_actions
      # Events for investigators
      @game_board.ready_for_events_investigators.reload.each do |i|
        result = i.ia_actions( @game_board, @prof )
        break if result == :break
      end
    end

    def investigators_move
      @game_board.ready_to_move_investigators.reload.each do |i|
          i.ia_play_movements( @game_board, @prof )
      end
      @game_board.inv_movement_done!
    end

    def some_investigator_is_ready_to_play?
      @game_board.ready_to_move_investigators.count + @game_board.ready_for_events_investigators.count > 0
    end

  end
end

