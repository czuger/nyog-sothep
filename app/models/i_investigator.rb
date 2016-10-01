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

  def translated_name
    I18n.t( "investigators.#{code_name}" )
  end

  def goes_back( game_board )
    ll = last_location
    self.current_location = ll
    # save!
    EEventLog.log_investigator_movement( game_board, self, ll, direction: :return )
  end

  def loose_san( game_board, san_amount )
    decrement!( :san, san_amount )
    EEventLog.log( game_board, I18n.t( 'actions.result.perte_san', san: san_amount,
      investigator_name: translated_name, final_san: san ) )
    if san <= 0
      die( game_board )
      return false
    end
    true
  end

  private

  def die( game_board )
    EEventLog.log( game_board, I18n.t( "actions.result.crazy.#{gender}", investigator_name: translated_name ) )
    game_board.p_monster_positions.create!(
      location: current_location, code_name: 'fanatiques', discovered: true )
    update_attribute( :dead, true )
  end
  
end
