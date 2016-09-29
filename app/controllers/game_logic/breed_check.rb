module GameLogic
  module BreedCheck

    private

    def can_breed_in_city?( position )
      !@game_board.p_monster_positions.exists?( location_id: position.id )
    end

    def assert_breed_validity( position )
      raise "Can only breed in city : #{position.inspect}" unless position.city?
      raise "Can't breed in occupied city : #{position.inspect}" unless can_breed_in_city?( position )
      raise "Deep one can only be breed in port : #{position.inspect}" unless position.city?
    end

  end
end
