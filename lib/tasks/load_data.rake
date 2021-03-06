namespace :load_data do

  desc 'Load all'
  task :all => [ :environment, :cities, :roads, :water_areas, :water_links ]


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