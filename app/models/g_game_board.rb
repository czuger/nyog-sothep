class GGameBoard < ApplicationRecord
  include AASM

  has_many :e_event_logs, dependent: :destroy
  has_many :i_investigators, dependent: :destroy
  has_many :m_monsters, dependent: :destroy
  has_many :p_monster_positions, dependent: :destroy
  has_many :p_monsters, dependent: :destroy
  has_many :p_prof_positions, dependent: :destroy
  has_one :p_professor, dependent: :destroy

  aasm do

    state :prof_move, :initial => true
    state :inv_move, :inv_event

    event :prof_move_end do
      transitions :from => :prof_move, :to => :inv_move, after: :reset_investigators
    end

    event :inv_move_end do
      transitions :from => :inv_move, :to => :inv_event
    end

    event :inv_event_end do
      transitions :from => :inv_event, :to => :prof_move
    end

    event :players_replay do
      transitions :from => :inv_event, :to => :inv_move
    end

  end

  def reset_investigators
    i_investigators.where( aasm_state: :event_phase_done ).each do |inv|
      inv.next_turn!
    end
  end

end
