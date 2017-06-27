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

    gender 'm'
    current_location_code_name :oxford
    last_location_code_name :oxford
    ia_target_destination_code_name :oxford

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

    factory :poirot do
      code_name 'poirot'
      aasm_state :turn_finished
    end

    factory :low_san_investigator do
      san 1
    end

    factory :repelling_investigator do
      san 30
      spell true
      current_location_code_name :oxford
    end

  end
end
