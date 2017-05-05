FactoryGirl.define do
  factory :p_monster_position do
    location nil
    token_rotation 5

    factory :fanatiques do
      code_name 'fanatiques'
    end
    factory :profonds do
      code_name 'profonds'
    end
    factory :goules do
      code_name 'goules'
    end
    factory :choses_brume do
      code_name 'choses_brume'
    end
    factory :habitants do
      code_name 'habitants'
    end

  end
end
