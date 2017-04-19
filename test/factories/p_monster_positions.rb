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
  end
end
