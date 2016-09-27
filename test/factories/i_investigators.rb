FactoryGirl.define do
  factory :i_investigator do
    sequence :code_name do |n|
      "Std investigator #{n}"
    end
    san 1
    weapon false
    sign false
    medaillon false
    spell false
    current true
    gender 'm'
    current_location { create( :c_city ) }
    factory :has_moved_investigator do
      sequence :code_name  do |n|
        "Has moved investigator #{n}"
      end
      aasm_state :roll_event
    end

  end
end
