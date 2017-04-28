class GGameBoard < ApplicationRecord
  include AASM

  include GameCore::Encounters

  has_many :e_event_logs, dependent: :destroy

  has_many :i_investigators, dependent: :destroy
  has_many :alive_investigators, -> { where.not( 'i_investigators.aasm_state' => :dead ) }, class_name: 'IInvestigator'
  has_many :ready_to_move_investigators, -> { where( 'i_investigators.aasm_state' => :move ).order( 'i_investigators.id' ) }, class_name: 'IInvestigator'
  has_many :ready_for_events_investigators, -> { where( 'i_investigators.aasm_state' => :events ).order( 'i_investigators.id' ) }, class_name: 'IInvestigator'

  has_many :m_monsters, dependent: :destroy
  has_many :p_monster_positions, dependent: :destroy
  has_many :p_monsters, dependent: :destroy
  has_many :i_inv_target_positions, dependent: :destroy
  has_one :p_professor, dependent: :destroy

  belongs_to :asked_fake_cities_investigator, class_name: 'IInvestigator', optional: true
  belongs_to :nyog_sothep_invocation_position, class_name: 'CCity'

  aasm do
    state :prof_move, :initial => true
    state :prof_asked_for_fake_cities, :inv_move, :inv_events

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

  end

  def next_turn
    ActiveRecord::Base.transaction do

      increment!( :turn )

      alive_investigators.each do |i|
        i.new_turn! unless i.in_misty_things?
      end

      # Prof positions are forgotten over time
      IInvTargetPosition.where( g_game_board_id: id ).update_all( 'memory_counter = memory_counter - 1' )
      IInvTargetPosition.where( g_game_board_id: id ).where( 'memory_counter <= 0' ).delete_all

      inv_events_done!

    end
  end

  def professor_pick_start_monsters
    1.upto( 5 ).each do
      p_professor.pick_one_monster
    end
  end


end
