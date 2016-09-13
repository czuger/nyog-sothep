Rails.application.routes.draw do

  resources :g_game_boards, only: [] do

    resource :maps, only: [:show] do
      post :switch_table
    end

    resource :actions, only: [] do
      patch :go_psy
      patch :move
    end

    get 'professor_map'

  end

  root 'maps#show'

  # For details on the DSL available within this file, see http://guides.rubyonrails.org/routing.html
end
