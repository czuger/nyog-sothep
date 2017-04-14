class GGameBoard < ApplicationRecord
  include AASM

  include GameCore::Encounters

  has_many :e_event_logs, dependent: :destroy

  has_many :i_investigators, dependent: :destroy
  has_many :alive_investigators, -> { where( dead: false ) }, dependent: :destroy, class_name: 'IInvestigator'

  has_many :m_monsters, dependent: :destroy
  has_many :p_monster_positions, dependent: :destroy
  has_many :p_monsters, dependent: :destroy
  has_many :i_inv_target_position, dependent: :destroy
  has_one :p_professor, dependent: :destroy

  aasm do
    state :prof_move, :initial => true
  end

  def professor_pick_start_monsters
    1.upto( 4 ).each do
      p_professor.pick_one_monster
    end
  end


end
