FactoryGirl.define do
  factory :l_log do
    turn 1

    event_translation_code 'test'
    event_translation_data { {} }
  end
end
