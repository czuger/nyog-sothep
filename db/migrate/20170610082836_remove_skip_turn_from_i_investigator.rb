class RemoveSkipTurnFromIInvestigator < ActiveRecord::Migration[5.0]
  def change
    remove_column :i_investigators, :skip_turns, :integer
    remove_column :i_investigators, :san_gain_after_lost_turns, :integer
  end
end
