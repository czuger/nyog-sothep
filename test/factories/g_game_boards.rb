FactoryGirl.define do
  factory :g_game_board do
    turn 1

    after(:create) do |gb|
      inv_road = create( :inv_road )
      create( :i_investigator, g_game_board_id: gb.id, current_location: inv_road.src_city, last_location: inv_road.src_city )
    end

    after(:create) do |gb|
      prof_road = create( :prof_road )
      create( :p_professor, g_game_board_id: gb.id, current_location: prof_road.src_city )
    end

    after(:create) do |gb|
      1.upto( 10 ).each do
        create( :m_monster, g_game_board_id: gb.id )
      end
    end

    factory :g_game_board_with_event_ready_investigators do
      after(:create) do |gb|
        inv_road = create( :inv_road )
        1.upto( 4 ).each do
          create( :has_moved_investigator, g_game_board_id: gb.id, current_location: inv_road.src_city, last_location: inv_road.src_city )
        end
      end
    end
  end
end
