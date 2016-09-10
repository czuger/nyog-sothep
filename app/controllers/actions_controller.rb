class ActionsController < ApplicationController

  include GameLogic::Turn
  include GameLogic::Dices

  def go_psy

    ActiveRecord::Base.transaction do

      EEventLog.start_event_block

      set_current_investigator

      @current_investigator.update_attribute( :delayed, true )
      san = d6
      @current_investigator.increment!( :san, san )
      EEventLog.log( I18n.t( "actions.result.psy.#{@current_investigator.gender}",
        san: san, investigator_name: t( "investigators.#{@current_investigator.code_name}" ) ) )

      set_next_investigator

    end

    redirect_to maps_url
  end

end
