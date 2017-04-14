class IInvTargetPosition < ApplicationRecord

  belongs_to :position, polymorphic: true

end
