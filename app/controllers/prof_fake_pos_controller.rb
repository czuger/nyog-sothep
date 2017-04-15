class ProfFakePosController < ApplicationController

  def new
    set_game_board

    @investigators = @game_board.alive_investigators
    @prof_location = @prof.current_location
    @monsters_positions = @game_board.p_monster_positions.all
    @nb_cities = params[ :nb_cities ].to_i

    @cities = CCity.all.reject{ |e| e == @prof_location }
  end

  def create
    set_game_board

    cities_ids = params[ :cities_ids ]
    assert( @game_board.prof_asked_for_fake_cities?, "@game_board.aasm_state = #{@game_board.aasm_state}" )
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
        IInvTargetPosition.create!( g_game_board_id: @game_board.id, position: city, memory_counter: 3 )
      end
    end

    redirect_to g_game_board_play_url( id: @game_board.id )
  end
end
