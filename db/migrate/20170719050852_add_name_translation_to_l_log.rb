class AddNameTranslationToLLog < ActiveRecord::Migration[5.0]
  def change
    add_column :l_logs, :name_translation_method, :string
  end
end
