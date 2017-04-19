FactoryGirl.define do
  factory :i_investigator do
    sequence :code_name do |n|
      "Std investigator #{n}"
    end
    san 500 # Otherwise full t
    weapon false
    sign false
    medaillon false
    spell false
    current true
    gender 'm'
    current_location { CCity.find_by( code_name: :oxford ) || create( :oxford ) }
    last_location { CCity.find_by( code_name: :oxford ) || create( :oxford ) }

    token_rotation 5

    factory :has_moved_investigator do
      sequence :code_name  do |n|
        "Has moved investigator #{n}"
      end
      aasm_state :events
    end

    factory :le_capitaine do
      code_name 'le_capitaine'
    end

  end
end
