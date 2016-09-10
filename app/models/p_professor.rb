class PProfessor < ApplicationRecord
  belongs_to :current_location, polymorphic: true
end
