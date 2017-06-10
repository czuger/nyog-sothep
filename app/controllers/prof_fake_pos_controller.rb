class ProfFakePosController < ApplicationController

  def new
    set_game_board

    @investigators = @game_board.alive_investigators
    @prof_location = @prof.current_location
    @monsters_positions = @game_board.p_monster_positions.all
    @nb_cities = @game_board.asked_fake_cities_count

    @nyog_sothep_location = @game_board.nyog_sothep_invocation_position
    @nyog_sothep_location_rotation = @game_board.nyog_sothep_invocation_position_rotation

    @cities = GameCore::Map::City.all.reject{ |c| c.code_name == @prof_location.code_name }

    set_position_x_decal
  end

  def create
    set_game_board

    cities_ids = params[ :cities_ids ]
    assert( @game_board.prof_asked_for_fake_cities?, "@game_board.aasm_state = #{@game_board.aasm_state}, should be prof_asked_for_fake_cities" )
    assert( @game_board.asked_fake_cities_count == cities_ids.count,
            "Bad city count#{@game_board.asked_fake_cities_count} != #{cities_ids.count}" )

    cities = cities_ids
    cities << @game_board.p_professor.current_location_code_name.to_s
    cities.uniq!

    # This case mean that somebody tried to send the prof location into the city_ids
    assert( @game_board.asked_fake_cities_count + 1 == cities.count,
            "#{@game_board.asked_fake_cities_count + 1} != #{cities.count}" )

    ActiveRecord::Base.transaction do
      cities.each do |city|
        IInvTargetPosition.find_or_create_by!( g_game_board_id: @game_board.id, position_code_name: city, memory_counter: 5 )
      end

      @game_board.return_to_move_status!

      # The we need to finish the Investigators IA play
      # At the end of the professor move, investigators play
      @game_board.investigators_ia_play( @prof )
    end

    redirect_to g_game_board_play_url( @game_board )
  end
end
