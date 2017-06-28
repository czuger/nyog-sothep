module GameCore
  module EventGroundB

    private

    def table2_e1( game_board, professor )
      event_log = log_event( game_board, __method__ )
      encounter_great_psy!
    end

    def table2_e2( game_board, _ )
      event_log = log_event( game_board, __method__ )
      update_attribute( :weapon, true ) if loose_san( game_board,  3, event_log )
    end

    def table2_e3( game_board, _ )
      event_log = log_event( game_board, __method__ )
      update_attribute( :spell, true ) if loose_san( game_board,  2, event_log )
    end

    # def table2_e4( game_board, professor )
    #   event_log = log_event( game_board, __method__ )
    #   replay!
    # end

    def table2_e5( game_board, _ )
      event_log = log_event( game_board, __method__ )
      update_attribute( :sign, true ) if loose_san( game_board,  2, event_log )
    end

    def table2_e6( game_board, _ )
      event_log = log_event( game_board, __method__ )
      ask_prof_for_fake_cities( game_board, 2 ) unless game_board.p_professor.current_location.water_area?
    end

    def table2_e7( game_board, _ )
      event_log = log_event( game_board, __method__ )
      goes_back( game_board )
    end

    def table2_e8( game_board, _ )
      event_log = log_event( game_board, __method__ )
      update_attribute( :medaillon, true ) if loose_san( game_board,  2, event_log )
    end

    def table2_e9( game_board, _ )
      event_log = log_event( game_board, __method__ )
      ask_prof_for_fake_cities( game_board, 1 ) unless game_board.p_professor.current_location.water_area?
    end

    # def table2_e10( game_board, professor )
    #   event_log = log_event( game_board, __method__ )
    #   if loose_san( game_board,  2, event_log )
    #     update_attribute( :spell, true )
    #     replay!
    #   end
    # end

    # def table2_e11( game_board, professor )
    #   event_log = log_event( game_board, __method__ )
    #   update_attribute( :weapon, true )
    #   replay!
    # end

    def table2_e12( game_board, _ )
      event_log = log_event( game_board, __method__ )
      loose_san( game_board,  2, event_log )
    end

    def table2_e13( game_board, professor )
      event_log = log_event( game_board, __method__ )
      professor.spotted( game_board )
    end

    # def table2_e17( game_board, professor )
    #   event_log = log_event( game_board, __method__ )
    #   replay!
    # end

    def table2_e18( game_board, _ )
      event_log = log_event( game_board, __method__ )
      update_attribute( :weapon, false ) if loose_san( game_board,  2, event_log )
    end

  end
end

