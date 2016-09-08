module GameLogic::Turn

  def set_current_investigator
    @current_investigator = IInvestigator.find_by( current: true )
  end

  def set_next_investigator

    begin
      cycle_investigators

      # If investigator is delayed, we remove the delayed flag and then cycle again to the next investigator
      investigator_delayed = @current_investigator.delayed
      @current_investigator.update_attribute( :delayed, false ) if investigator_delayed
    end while investigator_delayed

  end

  private

  def cycle_investigators
    next_investigator = IInvestigator.where( 'id > ?', @current_investigator.id ).order( :id ).first
    next_investigator = IInvestigator.order( :id ).first unless next_investigator

    @current_investigator.update_attribute( :current, false )
    next_investigator.update_attribute( :current, true )
    set_current_investigator
  end

end