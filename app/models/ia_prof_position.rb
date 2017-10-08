class IaProfPosition < ApplicationRecord
  belongs_to :g_game_board
  serialize :gb_data
end
