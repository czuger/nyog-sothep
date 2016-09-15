class IInvestigator < ApplicationRecord

  include AASM

  validates :event_table, inclusion: { in: [ 1, 2 ] }
  validates :gender, inclusion: { in: %w( m f ) }

  belongs_to :current_location, polymorphic: true
  belongs_to :last_location, polymorphic: true

  aasm do
    state :ready_to_move, :initial => true
    state :move_phase_done, :known_psy_help, :delayed, :event_phase_done

    event :moved do
      transitions :from => :ready_to_move, :to => :move_phase_done
    end

    event :didnt_move do
      transitions :from => :ready_to_move, :to => :event_phase_done
    end

    event :helped_by_kown_psy do
      transitions :from => :move_phase_done, :to => :known_psy_help
    end

    event :be_delayed do
      transitions :from => :move_phase_done, :to => :delayed
    end

    event :play_again do
      transitions :from => :move_phase_done, :to => :ready_to_move
    end

    event :finalize_event do
      transitions :from => [:move_phase_done, :known_psy_help, :delayed], :to => :event_phase_done
    end

    event :next_turn do
      transitions :from => :event_phase_done, :to => :ready_to_move
    end
  end

end
