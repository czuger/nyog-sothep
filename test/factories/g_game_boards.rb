FactoryGirl.define do
  factory :g_game_board do

    turn 1
    players_count 2

    nyog_sothep_invocation_position { CCity.find_by( code_name: :oxford ) || create( :oxford ) }
    nyog_sothep_invocation_position_rotation 5

    after(:create) do |gb|
      oxford = CCity.find_by( code_name: :oxford ) || create( :oxford )
      plainfield = CCity.find_by( code_name: :plainfield ) || create( :plainfield )
      providence = CCity.find_by( code_name: :providence ) || create( :providence )

      RRoad.find_by( src_city_id: oxford.id, dest_city_id: plainfield.id ) || create( :true_road )
      RRoad.find_by( src_city_id: plainfield.id, dest_city_id: oxford.id ) || create( :back_true_road )

      RRoad.find_by( src_city_id: providence.id, dest_city_id: plainfield.id ) || create( :plainfield_to_providence_back )
      RRoad.find_by( src_city_id: plainfield.id, dest_city_id: providence.id ) || create( :plainfield_to_providence )

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
    #   #   create( :i_investigator, g_game_board_id: gb.id, current_location: road.src_city, last_location: road.src_city )
    #   # end
    #   # create( :le_capitaine, g_game_board_id: gb.id, current_location: road.src_city, last_location: road.src_city )
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
        road = RRoad.first
        gb.i_investigators.destroy_all
        gb.reload
        create( :has_moved_investigator, g_game_board_id: gb.id, current_location: road.src_city, last_location: road.src_city )
      end
    end

    factory :g_game_board_for_inv_movement_tests do
      after(:create) do |gb|

        gb.i_investigators.destroy_all
        gb.reload
        1.upto( 4 ).each do
          create( :i_investigator, g_game_board_id: gb.id, current_location: road.src_city, last_location: road.src_city, ia_target_destination: road.dest_city )
        end
      end
    end

    factory :g_game_board_with_cross_border do
      after(:create) do |gb|
        RRoad.first.destroy
        road = create( :inv_cross_border_road )
        gb.i_investigators.destroy_all
        gb.reload
        1.upto( 4 ).each do
          create( :i_investigator, g_game_board_id: gb.id, current_location: road.src_city, last_location: road.src_city, ia_target_destination: road.dest_city )
        end
      end
    end

  end
end
