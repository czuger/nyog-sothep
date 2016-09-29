FactoryGirl.define do
  factory :w_water_area do
    sequence :code_name do |i|
      "Water area #{i}"
    end
    x 1
    y 1
  end
end
