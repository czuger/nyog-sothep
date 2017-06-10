class IInvestigator < ApplicationRecord

  include AASM

  include GameCore::Events
  include GameCore::Ia::Investigator
  include GameCore::IInvestigatorFight
  include GameCore::IInvestigatorTurn

  validates :event_table, inclusion: { in: [ 1, 2 ] }
  validates :gender, inclusion: { in: %w( m f ) }

  belongs_to :g_game_board

  aasm do
    state :move, :initial => true
    state :events, :turn_finished, :dead
    state :at_a_psy, :at_a_great_psy, :going_to_great_psy
    state :at_misty_things, :to_misty_things

    event :movement_done do
      transitions :from => :move, :to => :events
    end

    # Psy events
    event :going_to_psy do
      transitions :from => :move, :to => :at_a_psy
    end

    event :back_from_psy do
      transitions :from => :at_a_psy, :to => :move
    end

    # Great psy events
    event :encounter_great_psy do
      transitions :from => :events, :to => :going_to_great_psy
    end

    event :finishing_turn_at_a_great_psy do
      transitions :from => :going_to_great_psy, :to => :at_a_great_psy
    end

    event :back_from_great_psy do
      transitions :from => :at_a_great_psy, :to => :move, :after => Proc.new { |_| gain_san_from_great_psy() }
    end

    # Misty things event
    event :entering_misty_things do
      transitions :from => :events, :to => :to_misty_things
    end

    event :finishing_turn_in_misty_things do
      transitions :from => :to_misty_things, :to => :at_misty_things
    end

    event :back_from_misty_things do
      transitions :from => :at_misty_things, :to => :move, :after => Proc.new { |_| loose_san_from_misty_things( g_game_board ) }
    end

    # Other events
    event :events_done do
      transitions :from => :events, :to => :turn_finished
    end

    event :new_turn do
      transitions :from => [:turn_finished, :psy], :to => :move
    end

    # We already have a method called die, so we call it aasm_die to avoid confusion
    event :aasm_die do
      transitions :from => [:move, :events], :to => :dead
    end

    # event :skip_turn do
    #   transitions :from => :move, :to => :turn_finished
    # end

  end

  def translated_name
    I18n.t( "investigators.#{code_name}" )
  end

  def current_location
    GameCore::Map::Location.get_location( current_location_code_name )
  end

end
