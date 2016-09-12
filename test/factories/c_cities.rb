FactoryGirl.define do
  factory :c_city do
    x 1
    y 1
    factory :inv_city do
      code_name 'Inv city'
    end
    factory :inv_dest_city do
      code_name 'Inv dest city'
    end
    factory :prof_city do
      code_name 'Prof city'
    end
    factory :prof_dest_city do
      code_name 'Prof dest city'
    end
  end
end
