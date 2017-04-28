class AddSkipTurnsToIInvestigator < ActiveRecord::Migration[5.0]
  def change
    add_column :i_investigators, :skip_turns, :integer
    add_column :i_investigators, :san_gain_after_lost_turns, :integer
  end
end
