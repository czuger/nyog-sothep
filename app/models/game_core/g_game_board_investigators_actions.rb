module GameCore

  # Included in GGameBoard
  module GGameBoardInvestigatorsActions

    def investigators_ia_play( prof )

      prof_position_finder = GameCore::Ia::ProfPositionFinder.new
      prof_position_finder.load( self )

      ActiveRecord::Base.transaction do

        case aasm_state
          when 'inv_move'
            # The regular case, we move then we play events

            investigators_move( prof_position_finder )
            investigators_actions( prof, prof_position_finder )

          # This is where we go after a break (user asked for options)
          when 'inv_events'
            investigators_actions( prof, prof_position_finder )

          else
            raise "Bad aasm_state : #{aasm_state}"
        end

        self.save!

        # Once all investigators have played, we goes to next turn
        # TODO : is this working with misty_step caus based on events ... maybe could lead to infinite loop
        unless prof_asked_for_fake_cities? || some_investigator_is_ready_to_play?
          finalize_turn( prof_position_finder )
        end

        prof_position_finder.save( self )

      end
    end

    private

    def skip_investigators_turn
      skip_turns_investigators.reload.each do |i|
        print "Investigator state : #{i.code_name} : #{i.aasm_state}"
        i.decrement!( :skip_turns )
        i.gain_san( self, i.san_gain_after_lost_turns ) if i.san_gain_after_lost_turns && i.skip_turns <= 0
        i.skip_turn!
      end
    end

    def investigators_actions( prof, prof_position_finder )
      # Events for investigators
      ready_for_events_investigators.reload.each do |i|
        result = i.ia_actions( self, prof, prof_position_finder )
        break if result == :break
      end
    end

    def investigators_move( prof_position_finder )
      imt = GameCore::Ia::InvestigatorMovementTarget.new( self, prof_position_finder )
      ready_to_move_investigators.reload.each do |i|
          i.ia_play_movements( self, imt )
          i.save!
      end
      inv_movement_done!
    end

    def some_investigator_is_ready_to_play?
      ready_to_move_investigators.count + ready_for_events_investigators.count > 0
    end

  end
end

