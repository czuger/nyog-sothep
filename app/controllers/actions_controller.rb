class ActionsController < ApplicationController

  include GameLogic::Turn
  include GameLogic::Dices

  def go_psy

    ActiveRecord::Base.transaction do

      EEventLog.start_event_block

      set_current_investigator

      if @current_investigator.current_location.city?
        @current_investigator.pass_next_turn
        san = d6
        @current_investigator.increment!( :san, san )
        EEventLog.log( I18n.t( "actions.result.psy.#{@current_investigator.gender}",
                               san: san, investigator_name: t( "investigators.#{@current_investigator.code_name}" ) ) )

        set_next_investigator
      else
        raise "Psy asked in non city area : #{params.inspect}"
      end

    end

    redirect_to maps_url
  end

end
