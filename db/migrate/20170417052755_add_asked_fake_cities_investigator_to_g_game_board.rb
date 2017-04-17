class AddAskedFakeCitiesInvestigatorToGGameBoard < ActiveRecord::Migration[5.0]
  def change
    # add_reference :g_game_boards, :asked_fake_cities_investigator, foreign_key: { to_table: :i_investigators }, index: false
    add_reference :g_game_boards, :asked_fake_cities_investigator, index: false # foreign key causes destroy issues
  end
end
