namespace :load_data do

  desc 'Load all'
  task :all => [ :environment, :cities, :roads, :water_areas, :water_links, :create_investigators, :populate_monsters ]

  desc 'Create investigators'
  task :create_investigators => :environment do
    investigators = %w( poirot hercule hastings le_capitaine sandy lemon )
    gender = %w( m m m m f f )

    GGameBoard.destroy_all

    gb = GGameBoard.first
    gb = GGameBoard.create!( turn: 1 ) unless gb

    investigators.each_with_index do |investigator, index|
      puts 'Creating / updating ' + investigator.humanize
      i = gb.i_investigators.where( code_name: investigator ).first_or_initialize
      c = WWaterArea.find_by( code_name: :nantucket_sound )
      last_loc = CCity.find_by( code_name: :nantucket )
      i.current_location = c
      i.last_location = last_loc
      i.current = false
      i.gender = gender[index]
      i.san = 1.upto(3).map{ |e| rand( 1..6 ) }.reduce(&:+)
      i.save!
    end
    gb.i_investigators.first.update_attribute( :current, true )

    gb.create_p_professor!( hp: 14, current_location: CCity.all.sample )
  end

  desc 'Populate monsters'
  task :populate_monsters => :environment do
    # monsters = %w( goule profond fanatique chose_brume habitants reves tempete horreur_volante teleportation )
    # monsters_counts = [ 8, 8, 10, 3, 4, 6, 2, 1, 2 ]

    monsters = %w( goules profonds fanatiques chose_brume habitants reves tempete )
    monsters_counts = [ 8, 8, 10, 3, 4, 6, 2 ]

    gb = GGameBoard.first
    monsters.each_with_index do |monster, index|
      1.upto( monsters_counts[ index ] ).each do
        gb.m_monsters.create!( code_name: monster )
      end
    end

    1.upto( 4 ).each do
      m = gb.m_monsters.all.to_a.sample
      gb.p_monsters.create!( code_name: m.code_name )
      m.delete
    end
  end

  desc 'Load water links '
  task :water_links => :environment do
    File.open( 'work/water_areas_links.txt', 'r' ) do |f|
      record = []
      f.readlines.each do |line|
        line = line.chomp
        if line.empty?
          puts "About to create : #{record.inspect}"

          source_water_area = WWaterArea.find_by( code_name: record[0] )
          raise "Unknown water area for #{record[0]}" unless source_water_area
          record.shift

          record.each do |line|
            c = CCity.find_by( code_name: line )
            unless c
              w = WWaterArea.find_by( code_name: line )
              puts "About to insert : #{w.inspect}"
              if w
                source_water_area.connected_w_water_areas << w unless source_water_area.connected_w_water_areas.exists?( w.id )
                w.connected_w_water_areas << source_water_area unless w.connected_w_water_areas.exists?( source_water_area.id )
              end
            else
              source_water_area.ports << c
            end
          end

          record = []
        else
          record << line
        end
      end
    end
  end

  desc 'Load water areas data'
  task :water_areas => :environment do
    File.open( 'work/water_areas.txt', 'r' ) do |f|
      record = []
      f.readlines.each do |line|
        line = line.chomp
        if line.empty?
          puts "About to create : #{record.inspect}"

          r=WWaterArea.where( code_name: record[0] ).first_or_initialize
          r.x = record[1]
          r.y = record[2]
          r.save!

          record = []
        else
          record << line
        end
      end
    end
  end

  desc 'Load roads data'
  task :roads => :environment do
    File.open( 'work/roads.txt', 'r' ) do |f|
      record = []
      f.readlines.each do |line|
        line = line.chomp
        if line.empty?
          src_city_name = record.shift
          record.each do |dest|
            dest_city_name, border = dest.split
            border = ( border == 'true' )
            puts "About to create/update : #{src_city_name} -> #{dest_city_name}"
            src_city = CCity.find_by( code_name: src_city_name )
            dest_city = CCity.find_by( code_name: dest_city_name )
            puts "#{src_city.inspect}, #{dest_city.inspect}"

            r=RRoad.where(src_city_id: src_city.id, dest_city_id: dest_city.id ).first_or_initialize
            r.border = border
            r.save!

            r=RRoad.where(dest_city_id: src_city.id, src_city_id: dest_city.id ).first_or_initialize
            r.border = border
            r.save!
          end

          record = []
        else
          record << line
        end
      end
    end
  end

  desc 'Load cities data'
  task :cities => :environment do
    File.open( 'work/cities.txt', 'r' ) do |f|
      record = []
      f.readlines.each do |line|
        line = line.chomp
        if line.empty?
          puts "About to create : #{record.inspect}"

          r=CCity.where( code_name: record[0] ).first_or_initialize
          r.x = record[1]
          r.y = record[2]
          r.port = record[3] == 'true'
          r.save!

          record = []
        else
          record << line
        end
      end
    end
  end

end