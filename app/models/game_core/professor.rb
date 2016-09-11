module GameCore
  class Professor

    def initialize( professor )
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
        monster = PMonster.all.to_a.sample
        c_loc = @professor.current_location
        PMonsterPosition.create!( location: c_loc, code_name: monster.code_name )
        monster.delete
      end
    end

    def city_free
      c_loc = @professor.current_location
      if c_loc.city?
        unless PMonsterPosition.where( location_id: c_loc.id ).exists?
          return true
        end
      end
      false
    end
  end
end

