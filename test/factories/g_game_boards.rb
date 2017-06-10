FactoryGirl.define do
  factory :g_game_board do

    turn 1
    players_count 2

    nyog_sothep_invocation_position_code_name :oxford
    nyog_sothep_invocation_position_rotation 5

    after(:create) do |gb|
      create( :p_professor, g_game_board_id: gb.id )
    end

    after(:create) do |gb|
      1.upto( 10 ).each do
        create( :m_monster, g_game_board_id: gb.id )
      end
    end

    # No automatic game board population

    # Monsters on board should be created at demand
    # after(:create) do |gb|
    #   1.upto( 4 ).each do
    #     create( :p_monster, g_game_board_id: gb.id )
    #   end
    # end

    # after(:create) do |gb|
    #   road = RRoad.first
    #   # 1.upto(4).each do
    #   #   create( :i_investigator, g_game_board_id: gb.id, current_location_code_name: :oxford, last_location_code_name: :oxford )
    #   # end
    #   # create( :le_capitaine, g_game_board_id: gb.id, current_location_code_name: :oxford, last_location_code_name: :oxford )
    # end

    factory :g_game_board_ready_for_ia_play do
      aasm_state 'prof_move'
      players_count 1
    end

    factory :g_game_board_ready_for_fight do
      aasm_state 'prof_move'
    end

    factory :g_game_board_with_event_ready_to_move_investigators do
      aasm_state 'inv_move'
    end

    factory :g_game_board_with_event_ready_for_events_investigators do
      aasm_state 'inv_events'
      after(:create) do |gb|
        gb.i_investigators.destroy_all
        gb.reload
        create( :has_moved_investigator, g_game_board_id: gb.id, current_location_code_name: :oxford, last_location_code_name: :oxford )
      end
    end

    factory :g_game_board_for_inv_movement_tests do
      after(:create) do |gb|
        gb.i_investigators.destroy_all
        gb.reload
        1.upto( 4 ).each do
          create( :i_investigator, g_game_board_id: gb.id, current_location_code_name: :oxford, last_location_code_name: :oxford, ia_target_destination_code_name: :plainfield )
        end
      end
    end

    factory :g_game_board_with_cross_border do
      after(:create) do |gb|
        gb.i_investigators.destroy_all
        gb.reload
        1.upto( 4 ).each do
          create( :i_investigator, g_game_board_id: gb.id, current_location_code_name: :oxford, last_location_code_name: :oxford, ia_target_destination_code_name: :plainfield )
        end
      end
    end

  end
end
