Rails.application.routes.draw do

  resources :g_game_boards, only: [ :create, :new, :index, :destroy ] do

    resources :log, only:[ :index, :show ]

    resource :prof_fake_pos, only: [ :create ]

    resource :professor_actions, only: [] do
      member do
        get :move
        get :dont_move
        get ':investigator_id/attack', action: :attack, as: :attack
        get ':monster_id/breed', action: :breed, as: :monster_breed
        get :invoke_nyog_sothep
        get 'ia/show'
      end
    end

    get 'play', action: :play, controller: :map
    get 'game_lost', action: :game_lost, controller: :map

  end

  root 'g_game_boards#index'

  # For details on the DSL available within this file, see http://guides.rubyonrails.org/routing.html
end
