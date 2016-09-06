Rails.application.routes.draw do

  resource :maps, only: [:show, :update ]

  root 'maps#show'

  # For details on the DSL available within this file, see http://guides.rubyonrails.org/routing.html
end
