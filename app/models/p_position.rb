class PPosition < ApplicationRecord
  belongs_to :l_location, polymorphic: true
end
