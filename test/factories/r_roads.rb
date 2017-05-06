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

    factory :oxford_pascoag do
      src_city { CCity.find_by( code_name: :oxford ) || create( :oxford ) }
      dest_city { CCity.find_by( code_name: :pascoag ) || create( :pascoag ) }
    end

    factory :pascoag_woonsocket do
      src_city { CCity.find_by( code_name: :pascoag ) || create( :pascoag ) }
      dest_city { CCity.find_by( code_name: :woonsocket ) || create( :woonsocket ) }
    end

    factory :woonsocket_providence do
      src_city { CCity.find_by( code_name: :woonsocket ) || create( :woonsocket ) }
      dest_city { CCity.find_by( code_name: :providence ) || create( :providence ) }
    end

  end
end
