FactoryGirl.define do
  factory :r_road do
    src_city nil
    dest_city nil
    border false

    factory :inv_road do
      src_city { create( :inv_city ) }
      dest_city { create( :inv_dest_city ) }
      factory :inv_cross_border_road do
        border true
      end
    end

    factory :prof_road do
      src_city { create( :prof_city ) }
      dest_city { create( :prof_dest_city ) }
    end

    factory :true_road do
      src_city { create( :oxford ) }
      dest_city { create( :plainfield ) }
    end

  end
end
