class AddForbiddenCityToIInvestigator < ActiveRecord::Migration[5.0]
  def change
    add_reference :i_investigators, :forbidden_city, foreign_key: { to_table: :c_cities }, index: false
  end
end
