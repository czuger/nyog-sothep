FactoryGirl.define do
  factory :p_professor do
    hp 1
    token_rotation 5

    current_location { create( :prof_city ) }
  end
end
