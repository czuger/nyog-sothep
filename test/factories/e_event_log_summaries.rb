FactoryGirl.define do
  factory :e_event_log_summary do
    g_game_board nil
    i_investigator nil
    turn 1
    event_translation_code "MyString"
    event_translation_data "MyString"
    e_event_log nil
  end
end
