class EEventLog < ApplicationRecord

  belongs_to :g_game_board
  belongs_to :actor, polymorphic: true

  def self.log( game_board, actor, event )
    aasm_state = 'No state'
    aasm_state = actor.aasm_state unless actor.class == PProfessor
    game_board.e_event_logs.create!( turn: game_board.turn, actor: actor, actor_aasm_state: aasm_state, message: event )
  end

  def self.log_investigator_movement( game_board, investigator, dest_loc_code_name, direction: :goes )

    translated_dest_loc = I18n.t( "locations.#{dest_loc_code_name}", :default => "à #{dest_loc_code_name.to_s.humanize}" )
    investigator_name = I18n.t( "investigators.#{investigator.code_name}" )

    event = case direction
      when :goes
        I18n.t( 'movement.goes', who: investigator_name, where: translated_dest_loc )
      when :return
        I18n.t( 'movement.return', who: investigator_name, where: translated_dest_loc )
      else
        raise "Unknown direction : #{direction}. Can only be :goes or :return"
    end

    log( game_board,investigator, event )
  end

end

