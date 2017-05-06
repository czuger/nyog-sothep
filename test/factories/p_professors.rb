FactoryGirl.define do
  factory :p_professor do
    hp 1
    token_rotation 5

    current_location { CCity.find_by( code_name: :providence ) || create( :providence ) }
  end
end
