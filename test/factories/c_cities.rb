FactoryGirl.define do
  factory :c_city do
    x 1
    y 1

    factory :oxford do
      code_name 'oxford'
    end

    factory :plainfield do
      code_name 'plainfield'
    end

  end
end
