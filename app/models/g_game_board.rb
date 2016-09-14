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

    state :start, :initial => true
    state :prof_move, :prof_attack, :inv_move, :inv_event, :inv_fight_prof

    event :start_turn do
      transitions :from => :start, :to => :prof_move
    end

    event :prof_move_end do
      transitions :from => :prof_move, :to => :prof_attack
    end

    event :prof_attack_end do
      transitions :from => :prof_attack, :to => :inv_move
    end

    event :inv_move_end do
      transitions :from => :inv_move, :to => :inv_event
    end

    event :inv_event_end do
      transitions :from => :inv_event, :to => :inv_fight_prof
    end

    event :inv_fight_prof_end do
      transitions :from => :inv_fight_prof, :to => :start
    end

  end
end
