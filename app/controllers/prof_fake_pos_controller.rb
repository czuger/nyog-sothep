class ProfFakePosController < ApplicationController

  include GameLogic::ShowMap

  def new
    set_show_map_common_variables

    @nb_cities = @game_board.asked_fake_cities_count
    @cities = GameCore::Map::City.all.reject{ |c| c.code_name == @prof_location.code_name }
  end

  def create
    set_game_board

    cities_codes_names = params[ :cities_ids ]
    assert( @game_board.prof_asked_for_fake_cities?, "@game_board.aasm_state = #{@game_board.aasm_state}, should be prof_asked_for_fake_cities" )
    assert( @game_board.asked_fake_cities_count == cities_codes_names.count,
            "Bad city count#{@game_board.asked_fake_cities_count} != #{cities_codes_names.count}" )

    cities_codes_names.each.with_index(1) do |city_code_name, idx|
      update_last_fake_position_code_name(idx, city_code_name)
    end

    cities_codes_names << @prof.current_location_code_name.to_s
    cities_codes_names.uniq!

    # This case mean that somebody tried to send the prof location into the city_ids
    assert( @game_board.asked_fake_cities_count + 1 == cities_codes_names.count,
            "#{@game_board.asked_fake_cities_count + 1} != #{cities_codes_names.count}" )

    ActiveRecord::Base.transaction do
      trust_value = 1.0 / cities_codes_names.count
      cities_codes_names.each do |city|
        IInvTargetPosition.find_or_create_by!( g_game_board_id: @game_board.id, position_code_name: city, trust: trust_value )
      end

      @game_board.return_to_move_status!

      # The we need to finish the Investigators IA play
      # At the end of the professor move, investigators play
      @game_board.investigators_ia_play( @prof )
      @prof.save!
    end

    redirect_to g_game_board_play_url( @game_board )
  end

  private

  def update_last_fake_position_code_name( idx, city_code_name )
    if @prof.last_fake_position_1_code_name != city_code_name && @prof.last_fake_position_2_code_name != city_code_name
      save_location = "last_fake_position_#{idx}_code_name"
      @prof.update_attribute( save_location, city_code_name )
    end
  end

end
