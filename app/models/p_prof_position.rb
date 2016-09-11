class PProfPosition < ApplicationRecord
  belongs_to :position, polymorphic: true

  def self.set_random_positions( current_location, nb_loc )

    if current_location.class == WWaterArea
      prof_locations = WWaterArea.all
    else
      prof_locations = CCity.where.not( id: current_location.id ).to_a.sample( nb_loc-1 )
      prof_locations << current_location
    end

    prof_locations.each do |location|
      PProfPosition.create!( position: location )
    end

  end
end
