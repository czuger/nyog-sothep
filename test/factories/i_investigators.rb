FactoryGirl.define do
  factory :i_investigator do
    code_name 'Std investigator'
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
      aasm_state :move_done
    end

  end
end
