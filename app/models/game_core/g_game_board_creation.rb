module GameCore
  class GGameBoardCreation

    def self.populate_game_board( gb, prof_position )
      create_investigators( gb, prof_position )
      populate_monsters( gb )
    end

    def self.create_investigators( gb, prof_position_code_name )
      investigators = %w( poirot hercule hastings le_capitaine sandy lemon )
      gender = %w( m m m m f f )

      investigators.each_with_index do |investigator, index|
        # puts 'Creating / updating ' + investigator.humanize
        i = gb.i_investigators.where( code_name: investigator ).first_or_initialize

        i.current_location_code_name = :nantucket
        i.last_location_code_name = :nantucket
        i.ia_target_destination_code_name = :nantucket

        i.gender = gender[index]
        i.san = GameCore::Dices.d6( 3 )
        i.token_rotation = rand( rand( -15 .. 15 ) )
        i.save!
      end
      # gb.i_investigators.first.update_attribute( :current, true )

      gb.create_p_professor!( hp: 14, current_location_code_name: prof_position_code_name, token_rotation: rand( -15 .. 15 ) )
    end

    def self.populate_monsters( gb )
      # monsters = %w( goule profond fanatique chose_brume habitants reves tempete horreur_volante teleportation )
      # monsters_counts = [ 8, 8, 10, 3, 4, 6, 2, 1, 2 ]

      monsters = {
        goules: 8, profonds: 8, fanatiques: 10, choses_brume: 3, habitants: 4, reves: 6, tempete: 2, horreur_volante: 1 }
      # implemented_monsters = [ :reves, :goules ]
      implemented_monsters = [ :goules, :reves, :profonds, :fanatiques, :choses_brume, :habitants ]

      gb.m_monsters.delete_all
      gb.p_monsters.delete_all

      implemented_monsters.each do |monster|
        1.upto( monsters[monster] ).each do
          gb.m_monsters.create!( code_name: monster )
        end
      end

      gb.professor_pick_start_monsters
    end
  end
end
