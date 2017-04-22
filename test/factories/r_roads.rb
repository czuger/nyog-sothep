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

    factory :back_true_road do
      src_city { CCity.find_by( code_name: :plainfield ) || create( :plainfield ) }
      dest_city { CCity.find_by( code_name: :oxford ) || create( :oxford ) }
    end

    factory :plainfield_to_providence do
      src_city { CCity.find_by( code_name: :plainfield ) || create( :plainfield ) }
      dest_city { CCity.find_by( code_name: :providence ) || create( :providence ) }
    end

    factory :plainfield_to_providence_back do
      dest_city { CCity.find_by( code_name: :plainfield ) || create( :plainfield ) }
      src_city { CCity.find_by( code_name: :providence ) || create( :providence ) }
    end

  end
end
