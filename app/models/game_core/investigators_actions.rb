module GameCore
  class InvestigatorsActions

    def initialize( game_board )
      @game_board = game_board
    end

    def investigators_ia_play
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
            else
              raise "Bad aasm_state : #{@game_board.aasm_state}"
          end
        end

      end
    end

    private

    def some_investigator_is_ready_to_play?
      @game_board.ready_to_move_investigators.count + @game_board.ready_for_events_investigators.count > 0
    end

  end
end

