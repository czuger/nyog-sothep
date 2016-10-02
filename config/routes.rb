Rails.application.routes.draw do

  resources :g_game_boards do

    resources :investigators_actions, only: [] do
      member do
        get :move
        get :go_psy
        post :switch_table
      end
    end
    # get 'investigators_actions/roll_events'

    resource :professor_actions, only: [] do
      member do
        get :move
        get :dont_breed
        get ':investigator_id/attack', action: :attack, as: :attack
        get :dont_attack
        get ':monster_id/breed', action: :breed, as: :monster_breed
      end
    end

    get 'play', action: :show, controller: :map

  end

  root 'g_game_boards#index'

  # For details on the DSL available within this file, see http://guides.rubyonrails.org/routing.html
end
