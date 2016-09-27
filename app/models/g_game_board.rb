class GGameBoard < ApplicationRecord
  include AASM

  include GameCore::Professor

  has_many :e_event_logs, dependent: :destroy
  has_many :i_investigators, dependent: :destroy
  has_many :m_monsters, dependent: :destroy
  has_many :p_monster_positions, dependent: :destroy
  has_many :p_monsters, dependent: :destroy
  has_many :p_prof_positions, dependent: :destroy
  has_one :p_professor, dependent: :destroy

  aasm do

    state :prof_move, :initial => true
    state :inv_move, :inv_event, :prof_breed

    event :prof_move_end do
      transitions :from => :prof_move, :to => :prof_breed
    end

    event :prof_breed do
      transitions :from => :prof_breed, :to => :inv_move
    end

    event :inv_move_end do
      transitions :from => :inv_move, :to => :inv_event, after: Proc.new {|*args| switch_status_of_skipping_inv_turn(*args) }
    end

    event :inv_event_end do
      transitions :from => :inv_event, :to => :prof_move, after: Proc.new {|*args| reset_investigators_status(*args) }
    end

    event :someplayer_shoot_again do
      transitions :from => :inv_event, :to => :inv_move
    end

  end

  def next_investigator_ready_for_event
    investigator_in_event_phase.order( :id ).first
  end

  def next_moving_investigator
    investigator_in_move_phase.order( :id ).first
  end

  private

  def switch_status_of_skipping_inv_turn
    i_investigators.where( aasm_state: [ :inv_skip_turn ] ).each do |inv|
      inv.roll_no_event_after_passing_turn!
    end
  end

  def reset_investigators_status
    i_investigators.where( aasm_state: :roll_no_event ).each do |inv|
      inv.play_next_turn_after_passing_turn!
    end
    i_investigators.each do |inv|
      inv.inv_move! if inv.play_next_turn?
      inv.inv_skip_turn! if inv.skip_next_turn?
    end
  end

  def investigator_in_event_phase
    i_investigators.where( aasm_state: [ :roll_event, :roll_no_event ] )
  end

  def investigator_in_move_phase
    i_investigators.where( aasm_state: [ :inv_move ] )
  end

end
