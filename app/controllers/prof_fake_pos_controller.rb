class ProfFakePosController < ApplicationController

  include GameLogic::ShowMap

  def new
    set_show_map_common_variables

    @nb_cities = @game_board.asked_fake_cities_count
    @cities = GameCore::Map::City.all.reject{ |c| c.code_name == @prof_location.code_name }
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
