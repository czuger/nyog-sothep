class IInvestigator < ApplicationRecord

  include AASM

  validates :event_table, inclusion: { in: [ 1, 2 ] }
  validates :gender, inclusion: { in: %w( m f ) }

  belongs_to :current_location, polymorphic: true
  belongs_to :last_location, polymorphic: true

  aasm do
    state :inv_move, :initial => true
    state :inv_skip_turn, :roll_event, :roll_no_event, :replay, :play_next_turn, :skip_next_turn

    event :roll_event do
      transitions :from => :inv_move, :to => :roll_event
    end

    event :roll_no_event do
      transitions :from => :inv_move, :to => :roll_no_event
    end

    event :roll_no_event_after_passing_turn do
      transitions :from => :inv_skip_turn, :to => :roll_no_event
    end

    event :replay do
      transitions :from => :roll_event, :to => :replay
    end

    event :play_next_turn do
      transitions :from => :roll_event, :to => :play_next_turn
    end

    event :play_next_turn_after_passing_turn do
      transitions :from => :roll_no_event, :to => :play_next_turn
    end

    event :skip_next_turn do
      transitions :from => :roll_event, :to => :skip_next_turn
    end

    event :inv_move do
      transitions :from => :play_next_turn, :to => :inv_move
    end

    event :inv_move_after_replay do
      transitions :from => :replay, :to => :inv_move
    end

    event :inv_skip_turn do
      transitions :from => :skip_next_turn, :to => :inv_skip_turn
    end
  end

end
