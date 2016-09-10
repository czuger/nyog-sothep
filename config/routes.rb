Rails.application.routes.draw do

  resource :maps, only: [:show, :update]

  resource :actions, only: [] do
    patch :go_psy
  end

  root 'maps#show'

  # For details on the DSL available within this file, see http://guides.rubyonrails.org/routing.html
end
