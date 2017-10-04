class GGameBoard < ApplicationRecord
  include AASM

  include GameCore::Encounters
  include GameCore::GGameBoardInvestigatorsActions
  include GameCore::NyogSothep

  has_many :l_logs, -> { includes( :actor ).order( 'id DESC' ) }, dependent: :destroy
  has_many :l_logs_summaries, -> { includes( :actor ).where.not( event_translation_summary_code: nil ).order( 'id DESC' ) }, class_name: 'LLog'

  has_many :i_investigators, dependent: :destroy
  has_many :alive_investigators, -> { where.not( 'i_investigators.aasm_state' => :dead ) }, class_name: 'IInvestigator'
  has_many :ready_to_move_investigators, -> { where( 'i_investigators.aasm_state' => :move ).order( 'i_investigators.id' ) }, class_name: 'IInvestigator'
  has_many :ready_for_events_investigators, -> { where( 'i_investigators.aasm_state' => :events ).order( 'i_investigators.id' ) }, class_name: 'IInvestigator'
  has_many :skip_turns_investigators, -> { where( 'i_investigators.skip_turns > 0' ).order( 'i_investigators.id' ) }, class_name: 'IInvestigator'
  has_many :g_destroyed_cities, dependent: :destroy

  has_many :m_monsters, dependent: :destroy
  has_many :p_monster_positions, dependent: :destroy
  has_many :p_monsters, dependent: :destroy
  has_many :i_inv_target_positions, dependent: :destroy
  has_one :p_professor, dependent: :destroy

  belongs_to :asked_fake_cities_investigator, class_name: 'IInvestigator', optional: true

  aasm do
    state :prof_move, :initial => true
    state :prof_asked_for_fake_cities, :inv_move, :inv_events
    state :game_won, :game_lost

    event :prof_movement_done do
      transitions :from => :prof_move, :to => :inv_move
    end

    event :inv_movement_done do
      transitions :from => :inv_move, :to => :inv_events
    end

    event :inv_events_done do
      transitions :from => :inv_events, :to => :prof_move
    end

    event :ask_prof_for_fake_cities do
      transitions :from => :inv_events, :to => :prof_asked_for_fake_cities
    end

    event :return_to_move_status do
      transitions :from => :prof_asked_for_fake_cities, :to => :inv_events
    end

    event :win_game do
      transitions :from => :prof_move, :to => :game_won
    end

    event :loose_game do
      transitions :from => :prof_move, :to => :game_lost
    end

  end

  def next_turn
    ActiveRecord::Base.transaction do

      # Reload is required for some tests
      alive_investigators.reload.each do |i|
        i.next_turn
      end

      update_i_inv_target_positions
      increment!( :turn )

      inv_events_done!
    end
  end

  def update_i_inv_target_positions
    # Removing very low trusted positions
    # IInvTargetPosition.where( g_game_board_id: id ).where( 'trust < 0.1' ).delete_all # Let's see examples
    # Prof positions are forgotten over time
    IInvTargetPosition.where( g_game_board_id: id ).where( "turn < #{turn-5}" ).delete_all
    IInvTargetPosition.where( g_game_board_id: id ).update_all( 'trust = trust - 0.1' )
  end

  def professor_pick_start_monsters
    1.upto( 5 ).each do
      p_professor.pick_one_monster
    end
  end

  def nyog_sothep_position
    return GameCore::Map::Location.get_location( nyog_sothep_current_location_code_name ) if nyog_sothep_current_location_code_name
    GameCore::Map::Location.get_location( nyog_sothep_invocation_position_code_name )
  end


end
