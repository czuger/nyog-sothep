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
            investigators_move
            investigators_actions

            # This is where we go after a break (user asked for options)
          when 'inv_events'
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

    def investigators_actions

      # Events for investigators
      @game_board.alive_investigators.each do |i|

        if i.in_misty_things?
          i.exit_misty_things!
        elsif i.psy?
          # nothing
        elsif i.dead?
          raise 'WTF ???'
        else
          @game_board.resolve_encounter( i )

          if i.events?
            i.ia_play_events( @game_board, @prof )

            # Break out of the investigators loop
            if @game_board.prof_asked_for_fake_cities?
              # If investigator is still alive
              i.events_done! if i.events?
              break
            else
              # When the turn of the investigator is finished we need to check for a prof fight
              i.check_for_prof_to_fight_in_city( @game_board, @prof )
              i.events_done! if i.events?
            end
          end
        end
      end
    end

    def investigators_move
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

