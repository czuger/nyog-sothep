class LLog < ApplicationRecord

  belongs_to :g_game_board
  belongs_to :actor, polymorphic: true, optional: true

  serialize :event_translation_data

  # Write a log entry in the log table
  #
  # @param [GGameBoard] game_board the current game board
  # @param [Object] actor
  # @param [Boolean] summary
  # @param [String] event_translation_data
  # @param [Boolean] log_gender
  # @param [Boolean] log_gender_summary
  # @param [String] name_translation_method
  def self.log( game_board, actor, event_translation_code, summary = false, event_translation_data = {},
    log_gender = false, log_gender_summary = false, name_translation_method = nil )

    aasm_state = actor.aasm_state if actor.kind_of?( IInvestigator )

    log_hash = { turn: game_board.turn, actor: actor, event_translation_code: event_translation_code,
                 event_translation_data: event_translation_data, actor_aasm_state: aasm_state,
                 summary: summary, log_gender: log_gender, log_gender_summary: log_gender_summary,
                 name_translation_method: name_translation_method }

    game_board.l_logs.create!( log_hash )

  end

  def self.log_investigator_movement( game_board, investigator, dest_loc_code_name, direction: :goes )
    event = 'movement.' + direction.to_s
    log( game_board,investigator, event, false,{ dest_cn: dest_loc_code_name } )
  end

end