FactoryGirl.define do
  factory :i_investigator do
    code_name "MyString"
    san 1
    weapon false
    sign false
    medaillon false
    spell false
    current true
    gender 'm'
    current_location { create( :c_city ) }
  end
end
