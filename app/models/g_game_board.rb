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

    state :prof_turn, :initial => true
    state :inv_turn, :inv_actions, :inv_prof_fight

    event :prof_turn_end do
      transitions :from => :prof_turn, :to => :inv_turn
    end

    event :inv_turn_end do
      transitions :from => :inv_turn, :to => :inv_actions
    end

    event :inv_actions_end do
      transitions :from => :inv_actions, :to => :inv_prof_fight
    end

    event :inv_prof_fight_end do
      transitions :from => :inv_prof_fight, :to => :prof_turn
    end

  end
end
