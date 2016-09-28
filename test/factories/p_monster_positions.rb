FactoryGirl.define do
  factory :p_monster_position do
    location nil
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
