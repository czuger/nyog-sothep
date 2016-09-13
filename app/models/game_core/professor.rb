module GameCore
  class Professor

    def initialize( game_board, professor )
      @game_board = game_board
      @professor = professor
    end

    def play
      place_monster
      move_professor
    end

    private

    def move_professor
      # Move professor.
      loc = @professor.current_location
      @professor.current_location = loc.destinations.sample
      @professor.save!
      # EEventLog.log( "Prof se déplace : #{prof.current_location.inspect}" )

      PProfPosition.delete_all
    end

    def place_monster
      if city_free
        monster = @game_board.m_monsters.all.to_a.sample
        c_loc = @professor.current_location
        @game_board.p_monster_positions.create!( location: c_loc, code_name: monster.code_name )
        monster.delete
      end
    end

    def city_free
      c_loc = @professor.current_location
      if c_loc.city?
        unless @game_board.p_monster_positions.where( location_id: c_loc.id ).exists?
          return true
        end
      end
      false
    end

  end
end

