class PMonsterPosition < ApplicationRecord
  belongs_to :location, polymorphic: true
end
