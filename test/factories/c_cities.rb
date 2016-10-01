FactoryGirl.define do
  factory :c_city do
    x 1
    y 1
    sequence :code_name do |i|
      "Std city #{i}"
    end
    factory :inv_city do
      sequence :code_name do |i|
        "Inv city #{i}"
      end
    end
    factory :inv_dest_city do
      sequence :code_name do |i|
        "Inv dest city #{i}"
      end
    end
    factory :prof_city do
      sequence :code_name do |i|
        "Prof city #{i}"
      end
    end
    factory :prof_dest_city do
      sequence :code_name do |i|
        "Prof dest city #{i}"
      end
    end
  end
end
