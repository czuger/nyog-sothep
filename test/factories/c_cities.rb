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

    factory :providence do
      code_name 'providence'
    end

    factory :pascoag do
      code_name 'pascoag'
    end

    factory :woonsocket do
      code_name 'woonsocket'
    end

  end
end
