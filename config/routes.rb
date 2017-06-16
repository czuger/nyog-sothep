Rails.application.routes.draw do

  resources :g_game_boards, only: [ :create, :new, :index, :destroy ] do

    resources :log, only:[ :index, :show ]

    resource :prof_fake_pos, only: [ :create, :new ]

    resource :professor_actions, only: [] do
      member do
        get :move
        get ':investigator_id/attack', action: :attack, as: :attack
        get ':monster_id/breed', action: :breed, as: :monster_breed
        get :invoke_nyog_sothep
      end
    end

    get 'play', action: :show, controller: :map

  end

  root 'g_game_boards#index'

  # For details on the DSL available within this file, see http://guides.rubyonrails.org/routing.html
end
