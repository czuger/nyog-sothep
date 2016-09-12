FactoryGirl.define do
  factory :p_professor do
    hp 1
    current_location { create( :prof_city ) }
  end
end
