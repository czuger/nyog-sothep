module GameCore
  module EventGroundB

    private

    def table2_e1( game_board, _ , _ )
      log_event( game_board, __method__ )
      encounter_great_psy!
    end

    def table2_e2( game_board, _ , _ )
			loose_san_from_event( game_board, __method__, 3 )
      update( weapon: true )
    end

    def table2_e3( game_board, _ , _ )
			loose_san_from_event( game_board, __method__, 2 )
      update( spell: true )
    end

    # def table2_e4( game_board, professor )
    #   log_event( game_board, __method__ )
    #   replay!
    # end

    def table2_e5( game_board, _ , _ )
			loose_san_from_event( game_board, __method__, 2 )
      update( sign: true )
    end

    def table2_e6( game_board, professor, _ )
      log_event( game_board, __method__ )
      ask_prof_for_fake_cities( game_board, 2 ) unless professor.current_location.water_area?
    end

    def table2_e7( game_board, _ , _ )
      log_event( game_board, __method__ )
      goes_back( game_board )
    end

    def table2_e8( game_board, _ , _ )
			loose_san_from_event( game_board, __method__, 2 )
      update( medaillon: true )
    end

    def table2_e9( game_board, professor, _ )
      log_event( game_board, __method__ )
      ask_prof_for_fake_cities( game_board, 1 ) unless professor.current_location.water_area?
    end

    # def table2_e10( game_board, professor )
    #   log_event( game_board, __method__ )
    #   if loose_san( game_board,  2 )
    #     update_attribute( :spell, true )
    #     replay!
    #   end
    # end

    # def table2_e11( game_board, professor )
    #   log_event( game_board, __method__ )
    #   update_attribute( :weapon, true )
    #   replay!
    # end

    def table2_e12( game_board, _ , _ )
      log_event( game_board, __method__ )
      loose_san( game_board,  2 )
    end

    def table2_e13( game_board, professor, prof_position_finder )
      log_event( game_board, __method__ , {}, true )
      professor.spotted( game_board, self, prof_position_finder )
    end

    # def table2_e17( game_board, professor )
    #   log_event( game_board, __method__ )
    #   replay!
    # end

    def table2_e18( game_board, _ , _ )
			loose_san_from_event( game_board, __method__, 2 )
      update( weapon: false )
    end

  end
end

