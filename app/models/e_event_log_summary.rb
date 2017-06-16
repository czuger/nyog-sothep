class EEventLogSummary < ApplicationRecord
  belongs_to :g_game_board
  belongs_to :actor, polymorphic: true
  belongs_to :e_event_log

  serialize :event_translation_data

  def self.log( game_board, actor, event_translation_code, event_translation_data, associated_event_log )
    game_board.e_event_log_summaries.create!(
      turn: game_board.turn, actor: actor, event_translation_code: event_translation_code,
      event_translation_data: event_translation_data, e_event_log_id: associated_event_log.id )
  end

end