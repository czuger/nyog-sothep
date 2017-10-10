# This class is used for the IA to track informations on the prof (spotted placer or guessed places)
class GameCore::Ia::ProfPositionFinder

  attr_reader :data

  def initialize
    # Load previous data
    @data = {}

    # This is used to avoid unnecessary Bfs computations
    @cached_data = nil
  end

  def load( game_board )
    @data = game_board.ia_prof_position.gb_data if game_board.ia_prof_position
  end

  def save( game_board )
    @data.delete_if {|key, _| key < game_board.turn-3 }

    if game_board.ia_prof_position
      game_board.ia_prof_position.gb_data = @data
      game_board.ia_prof_position.save!
    else
      game_board.create_ia_prof_position( gb_data: @data )
    end
  end

  # This method return the prof positions
  #
  # @param [Integer] turn the current turn
  # @return [Array[String]] one position mean we are sure. Multiple position mean we have to send investigators everywhere. Nil mean we have no info (walk randomly)
  def get_prof_positions( turn )

    # Avoiding too much computation
    return @cached_data if @cached_data

    if @data[ turn ]
      # If the prof is spotted, bingo
      if @data[ turn ][ :spotted ]
        return set_and_return_new_cached_data(@data[ turn ][ :spotted ])
      end

      # We only have fake pos, it is better than nothing. Maybe we have only one fake pos because the player is dumb
      if @data[ turn ][ :fake_pos ]
        return set_and_return_new_cached_data(@data[ turn ][ :fake_pos ])
      end
    end

    # We have nothing this turn. Guess from the previous turn.
    if @data[ turn-1 ]
      return set_and_return_new_cached_data(cities_around_last_spotted_cities( turn ))
    end

    # We have nothing this turn nor the previous turn. Guess from the penultimate turn. Criminals always go back to the crime scene.
    if @data[ turn-2 ]
      data = []
      data += @data[ turn-2 ][ :spotted ] if @data[ turn-2 ][ :spotted ]
      data += @data[ turn-2 ][ :fake_position ] if @data[ turn-2 ][ :fake_position ]
      return set_and_return_new_cached_data(data)
    end

    # Whe have nothing since 3 turn. We have nothing. Move randomly.
  end

  # This method is called when the prof is spotted
  #
  # @param [Integer] turn the current turn
  # @param [String] prof_position_code_name the code name of the position of the prof
  def spot_prof( turn, prof_position_code_name )
    raise "prof_position_code_name is not a string" unless prof_position_code_name.is_a?( String )
    @data[ turn ] ||= {}
    @data[ turn ][ :spotted ] ||= [ prof_position_code_name ]
    @cached_data = nil
  end

  # This method is called when some fake positions are given
  #
  # @param [Integer] turn the current turn
  # @param [Array[String]] fake_positions_code_names an array of strings containing the codes_names of the fakes positions
  def add_fake_pos( turn, prof, fake_positions_code_names )
    fake_positions_code_names << prof.current_location_code_name
    @data[ turn ] ||= {}
    @data[ turn ][ :fake_pos ] ||= fake_positions_code_names
    @data[ turn ][ :fake_pos ] = fake_positions_code_names & @data[ turn ][ :fake_pos ]
    validate_fake_pos_with_previous_turn_data( turn )
    @cached_data = nil
  end

  private

  def set_and_return_new_cached_data( data_array )
    @cached_data = data_array
  end

  # This method return the cities around the previous spotted or fakes cities
  #
  # @param [Integer] turn the current turn
  # @return [Array[String]] positions around old fakes_positions and spotted position. Nil if no last turn
  def cities_around_last_spotted_cities( turn )
    if @data[ turn-1 ]
      old_spotted_positions = []
      old_spotted_positions += @data[ turn-1 ][ :spotted ] if @data[ turn-1 ][ :spotted ]
      old_spotted_positions += @data[ turn-1 ][ :fake_pos ] if @data[ turn-1 ][ :fake_pos ]

      cities_around_last_spotted_cities_array = []
      old_spotted_positions.each do |old_spotted_position|
        nearby_cities = GameCore::Ia::BfsAlgo.find_cities_around_city( old_spotted_position, 1 )
        cities_around_last_spotted_cities_array += nearby_cities[1].map( &:to_s ) if nearby_cities
      end
      cities_around_last_spotted_cities_array + old_spotted_positions
    end
  end

  # This method check if positions are not too far from the previous spotted position and the previous fakes positions
  def validate_fake_pos_with_previous_turn_data( turn )
    if @data[ turn-1 ]
      @data[ turn ][ :fake_pos ] = @data[ turn ][ :fake_pos ] & cities_around_last_spotted_cities( turn )
    end
  end
end