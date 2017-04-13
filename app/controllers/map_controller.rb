require 'pp'

class MapController < ApplicationController

  include GameLogic::Events
  include GameLogic::BreedCheck
  include GameLogic::ProfFight

  def show
    set_game_board

    @investigators = @game_board.i_investigators
    @events = @game_board.e_event_logs.all.order( 'logset DESC, id ASC' )
    @prof_maybe_positions = @game_board.p_prof_positions.all
    @prof = @game_board.p_professor
    @prof_location = @prof.current_location

    main_loop

    # if ( @side == 'prof' && ( @game_board.prof_move? || @game_board.prof_attack? || @game_board.prof_fall_back? ||
    #   @game_board.inv_repelled? || @game_board.prof_move? ) ) || ( @side == 'inv' && ( @game_board.inv_move? || @game_board.inv_event? ) )
    #   main_loop
    # else
    #   @other_side_play = true
    # end

    # puts @side
  end


  private

  def main_loop
    ActiveRecord::Base.transaction do

      case_switch

    end
  end

  def case_switch

    prof_move

    # case @game_board.aasm_state
    #   when 'prof_move' then
    #     prof_move
    #   when 'prof_attack' then
    #     prof_attack
    #   when 'prof_fall_back' then
    #     prof_move
    #   when 'inv_repelled' then
    #     inv_repelled
    #   when 'prof_breed' then
    #     prof_breed
    #   when 'inv_move' then
    #     inv_move
    #   when 'inv_event' then
    #     process_events
    #
    #   else raise "Show case non implemented : #{@game_board.aasm_state}"
    # end
  end

  def inv_repelled
    @inv_to_attack = @game_board.i_investigators.find( params[ :attacking_investigator_id ] ) unless @inv_to_attack
    @inv_to_attack.current_location = CCity.where.not( id: @inv_to_attack.current_location_id ).sample
    @inv_to_attack.save!
    @game_board.prof_attack_after_inv_repelled!
    @loop = true
  end

  def prof_attack
    @prof_choose_attack, @inv_to_attack = @prof.prof_has_to_choose_attack?
    @loop = true unless @prof_choose_attack
  end

  def inv_move
    if( @current_investigator = @game_board.next_moving_investigator )
      @zone = @current_investigator.current_location
      @monsters_positions = @game_board.p_monster_positions.where( discovered: true )
      @aval_destinations = @zone.destinations
      @prof_spotted = @prof.spotted
      @inv_move = true
    else
      @game_board.inv_event!
      @loop = true
    end
  end

  def prof_move

    # unless @prof_move
      @monsters_positions = @game_board.p_monster_positions.all
      @prof_monsters = @game_board.p_monsters

      @aval_destinations = @prof_location.destinations
      @prof_move = true
    # end
  end

  def prof_breed

    @monsters_positions = @game_board.p_monster_positions.all
    @prof_monsters = @game_board.p_monsters

    prof_current_location = @prof.current_location
    @prof_in_water = !prof_current_location.city?
    @prof_in_port = prof_current_location.port if prof_current_location.city?

    if @prof.can_breed_in_city?( prof_current_location )
      @prof_breed = true
    else
      @game_board.inv_move!
      @loop = true
    end

  end
end
