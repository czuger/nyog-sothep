FactoryGirl.define do
  factory :r_road do
    src_city nil
    dest_city nil
    border false

    factory :true_road do
      src_city { CCity.find_by( code_name: :oxford ) || create( :oxford ) }
      dest_city { CCity.find_by( code_name: :plainfield ) || create( :plainfield ) }
      factory :inv_cross_border_road do
        border true
      end
    end

  end
end
