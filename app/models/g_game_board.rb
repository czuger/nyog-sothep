class GGameBoard < ApplicationRecord
  include AASM

  include GameCore::Encounters

  has_many :e_event_logs, dependent: :destroy

  has_many :i_investigators, dependent: :destroy
  has_many :alive_investigators, -> { where( dead: false ) }, class_name: 'IInvestigator'
  has_many :ready_to_play_investigators, -> { where( dead: false, current: true ) }, class_name: 'IInvestigator'

  has_many :m_monsters, dependent: :destroy
  has_many :p_monster_positions, dependent: :destroy
  has_many :p_monsters, dependent: :destroy
  has_many :i_inv_target_positions, dependent: :destroy
  has_one :p_professor, dependent: :destroy

  belongs_to :asked_fake_cities_investigator, class_name: 'IInvestigator', optional: true

  aasm do
    state :prof_move, :initial => true
    state :prof_asked_for_fake_cities

    event :ask_prof_for_fake_cities do
      transitions :from => :prof_move, :to => :prof_asked_for_fake_cities
    end

    event :return_to_move_status do
      transitions :from => :prof_asked_for_fake_cities, :to => :prof_move
    end

  end

  def professor_pick_start_monsters
    1.upto( 4 ).each do
      p_professor.pick_one_monster
    end
  end


end
