class ProfFakePosController < ApplicationController

  def new
    set_game_board

    @investigators = @game_board.alive_investigators
    @prof_location = @prof.current_location
    @monsters_positions = @game_board.p_monster_positions.all
    @nb_cities = @game_board.asked_fake_cities_count

    @cities = CCity.all.reject{ |e| e == @prof_location }
  end

  def create
    set_game_board

    cities_ids = params[ :cities_ids ]
    assert( @game_board.prof_asked_for_fake_cities?, "@game_board.aasm_state = #{@game_board.aasm_state}, should be prof_asked_for_fake_cities" )
    assert( @game_board.asked_fake_cities_count == cities_ids.count,
            "#{@game_board.asked_fake_cities_count} != #{cities_ids.count}" )

    cities = CCity.find( cities_ids )
    cities << @game_board.p_professor.current_location
    cities.uniq!

    # This case mean that somebody tried to send the prof location into the city_ids
    assert( @game_board.asked_fake_cities_count + 1 == cities.count,
            "#{@game_board.asked_fake_cities_count + 1} != #{cities.count}" )

    ActiveRecord::Base.transaction do
      cities.each do |city|
        IInvTargetPosition.find_or_create_by!( g_game_board_id: @game_board.id, position: city, memory_counter: 5 )
      end
    end

    @game_board.return_to_move_status!

    redirect_to g_game_board_investigators_ia_play_url( g_game_board_id: @game_board.id )
  end
end
