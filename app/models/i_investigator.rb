class IInvestigator < ApplicationRecord

  include AASM

  validates :event_table, inclusion: { in: [ 1, 2 ] }
  validates :gender, inclusion: { in: %w( m f ) }

  belongs_to :current_location, polymorphic: true

  aasm do
    state :normal, :initial => true
    state :delayed
    state :replay
    state :great_psy_help
    state :car_breakdown

    event :pass_next_turn do
      transitions :from => :normal, :to => :delayed
    end

    event :play_again do
      transitions :from => :normal, :to => :replay
    end

    event :see_psy do
      transitions :from => :normal, :to => :great_psy_help
    end

    event :car_break_down do
      transitions :from => :normal, :to => :car_breakdown
    end

    event :reset do
      transitions from: [:delayed, :replay, :great_psy_help, :car_breakdown], to: :normal
    end

  end

end
