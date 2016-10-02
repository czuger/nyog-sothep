module GameCore
  class GGameBoardCreation

    def self.populate_game_board( gb )
      create_investigators( gb )
      populate_monsters( gb )
    end

    def self.create_investigators( gb )
      investigators = %w( poirot hercule hastings le_capitaine sandy lemon )
      gender = %w( m m m m f f )

      investigators.each_with_index do |investigator, index|
        puts 'Creating / updating ' + investigator.humanize
        i = gb.i_investigators.where( code_name: investigator ).first_or_initialize
        c = WWaterArea.find_by( code_name: :nantucket_sound )
        last_loc = CCity.find_by( code_name: :nantucket )
        i.current_location = c
        i.last_location = last_loc
        i.current = false
        i.gender = gender[index]
        i.san = GameCore::Dices.d6( 3 )
        i.save!
      end
      gb.i_investigators.first.update_attribute( :current, true )

      gb.create_p_professor!( hp: 14, current_location: CCity.all.sample )
    end

    def self.populate_monsters( gb )
      # monsters = %w( goule profond fanatique chose_brume habitants reves tempete horreur_volante teleportation )
      # monsters_counts = [ 8, 8, 10, 3, 4, 6, 2, 1, 2 ]

      monsters = {
        goules: 8, profonds: 8, fanatiques: 10, chose_brume: 3, habitants: 4, reves: 6, tempete: 2, horreur_volante: 1 }
      # implemented_monsters = [ :reves, :goules ]
      implemented_monsters = [ :goules, :reves, :profonds, :fanatiques ]

      EEventLog.start_event_block( gb )

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
