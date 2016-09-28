Rails.application.routes.draw do

  resources :g_game_boards, only: [] do

    resource :map, only: [] do
      post :switch_table
    end

    resources :investigators_actions, only: [] do
      member do
        get :move
        get :go_psy
        # get :special_event
      end
    end
    # get 'investigators_actions/roll_events'

    resource :professor_actions, only: [] do
      member do
        get :move
        get :dont_breed
        get ':monster_id/breed', action: :breed, as: :monster_breed
      end
    end

  end


  get 'map/show'

  root 'map#show'

  # For details on the DSL available within this file, see http://guides.rubyonrails.org/routing.html
end
