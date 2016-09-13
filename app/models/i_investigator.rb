class IInvestigator < ApplicationRecord

  include AASM

  validates :event_table, inclusion: { in: [ 1, 2 ] }
  validates :gender, inclusion: { in: %w( m f ) }

  belongs_to :current_location, polymorphic: true
  belongs_to :last_location, polymorphic: true

  aasm do
    state :ready, :initial => true
    state :move_done, :known_psy_help, :delayed, :replay

    event :move do
      transitions :from => :ready, :to => :move_done
    end

    event :helped_by_kown_psy do
      transitions :from => :move_done, :to => :known_psy_help
    end

    event :be_delayed do
      transitions :from => :move_done, :to => :delayed
    end

    event :play_again do
      transitions :from => :move_done, :to => :replay
    end

    event :be_ready do
      transitions :from => [ :move_done, :known_psy_help, :delayed, :replay ], :to => :ready
    end

  end

end
