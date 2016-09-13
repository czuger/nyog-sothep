Rails.application.routes.draw do

  resources :g_game_boards, only: [] do

    resource :investigators_map, only: [] do
      post :switch_table
    end

    resources :investigators_actions, only: [] do
      member do
        get :move
        get :go_psy
      end
    end

    resource :professor_actions, only: [] do
      member do
        get :move
      end
    end

  end


  get 'professor_map/show'
  get 'investigators_map/show'

  root 'professor_map#show'

  # For details on the DSL available within this file, see http://guides.rubyonrails.org/routing.html
end
