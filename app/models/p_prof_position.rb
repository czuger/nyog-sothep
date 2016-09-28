class PProfPosition < ApplicationRecord
  belongs_to :position, polymorphic: true

  def self.set_random_positions( game_board, current_location, nb_loc )

    if current_location.class == WWaterArea
      prof_locations = WWaterArea.all
    else
      used_locations = game_board.p_prof_positions.pluck( :position_id )
      used_locations << current_location.id

      prof_locations = CCity.where.not( id: used_locations ).to_a.sample( nb_loc-1 )
      prof_locations << current_location unless game_board.p_prof_positions.where( position_id: current_location.id ).exists?
    end

    prof_locations.each do |location|
      game_board.p_prof_positions.create!( position: location )
    end

  end
end
