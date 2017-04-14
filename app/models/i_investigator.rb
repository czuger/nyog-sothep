class IInvestigator < ApplicationRecord

  include AASM

  include GameCore::Events
  include GameCore::Ia::Investigator


  validates :event_table, inclusion: { in: [ 1, 2 ] }
  validates :gender, inclusion: { in: %w( m f ) }

  belongs_to :current_location, polymorphic: true
  belongs_to :last_location, polymorphic: true
  belongs_to :ia_target_destination, polymorphic: true, optional: true

  aasm do
    state :inv_move, :initial => true
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
