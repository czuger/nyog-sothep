class ProfessorActionsController < ApplicationController

  def breed
    set_game_board
    raise "Prof breed called while game_board not in prof_breed state : #{@game_board.inspect}" unless @game_board.prof_breed?

    monster = PMonster.find( params[ :monster_id ] )
    raise "Unable to find monster for #{params.inspect}" unless monster

    ActiveRecord::Base.transaction do
      monster_loc = @game_board.p_professor.current_location
      @game_board.p_monster_positions.create!( location: monster_loc, code_name: monster.code_name )
      monster.destroy!
      @game_board.prof_breed!
      @game_board.professor_pick_one_monster
      # TODO : need to check that prof has no more than 5 monsters.
      # Need to ask him to remove one if this is the case
    end

    redirect_to map_show_url
  end

  def move
    set_game_board
    raise "Prof move called while game_board not in prof_move state : #{@game_board.inspect}" unless @game_board.prof_move?

    if params['zone_id'] && params['zone_class']
      dest_loc = params['zone_class'].constantize.find( params['zone_id'] )
    else
      raise "Zone error : #{params.inspect}"
    end

    ActiveRecord::Base.transaction do
      @game_board.p_professor.current_location = dest_loc
      @game_board.p_professor.save!
      @game_board.prof_move_end!
    end

    redirect_to map_show_url
  end

end
