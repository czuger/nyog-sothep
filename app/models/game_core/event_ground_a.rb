module GameCore
  module EventGroundA

    private

    def table1_e1( game_board, _ , _ )
      log_event( game_board, __method__ )
      goes_back( game_board )
    end

    def table1_e2( game_board, _ , _ )
			loose_san_from_event( game_board, __method__, :weapon, 2 )
    end

    def table1_e3( game_board, _ , _ )
			loose_san_from_event( game_board, __method__, :sign, 2 )
    end

    def table1_e4( game_board, _ , _ )
      log_event( game_board, __method__ )
      encounter_great_psy!
    end

    def table1_e5( game_board, _ , _ )
      log_event( game_board, __method__ )
      ask_prof_for_fake_cities( game_board, 2 ) unless game_board.p_professor.current_location.water_area?
    end

    def table1_e6( game_board, _ , _ )
			loose_san_from_event( game_board, __method__, :spell, 2 )
    end

    def table1_e7( game_board, _ , _ )
			loose_san_from_event( game_board, __method__, :weapon, 2 )
    end

    # def table1_e8( game_board, self, professor )
    #   log_event( game_board, __method__ )
    #   car_break_down!
    # end

    # def table1_e9( game_board, self, professor )
    #   log_event( game_board, __method__ )
    #   update_attribute( :medaillon, true )
    #   replay!
    # end

    def table1_e10( game_board, _ , _ )
      log_event( game_board, __method__ )
      ask_prof_for_fake_cities( game_board, 1 ) unless game_board.p_professor.current_location.water_area?
    end

    def table1_e11( game_board, _ , _ )
			loose_san_from_event( game_board, __method__, :weapon, 2 )
    end

    def table1_e12( game_board, _ , _ )
			loose_san_from_event( game_board, __method__, :spell, 3 )
    end

    def table1_e13( game_board, professor, prof_position_finder )
      log_event( game_board, __method__ )
      professor.spotted( game_board, prof_position_finder )
    end

    # def table1_e14( game_board, _ , _ )
    #   log_event( game_board, __method__ )
    #   replay!
    # end

    def table1_e18( game_board, professor, prof_position_finder )
      log_event( game_board, __method__ )
      professor.spotted( game_board, prof_position_finder )
    end

  end
end

