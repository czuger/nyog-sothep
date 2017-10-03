FactoryGirl.define do
  factory :l_log do
    turn 1
    sequence :summary do |n|
      "Summary #{n}"
    end
    event_translation_code 'test'
    event_translation_data { {} }
  end
end
