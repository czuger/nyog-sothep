class IInvestigator < ApplicationRecord
  include AASM

  belongs_to :current_location, polymorphic: true

  aasm do
    state :normal, :initial => true
    state :delayed
    state :replay
    state :delayed_and_increase_san

    event :delay do
      transitions :from => :normal, :to => :delayed
    end

    event :play_again do
      transitions :from => :normal, :to => :replay
    end

    event :see_psy do
      transitions :from => :normal, :to => :delayed_and_increase_san
    end

    event :reset do
      transitions from: [:delayed, :replay, :delayed_and_increase_san], to: :normal
    end

  end

end
