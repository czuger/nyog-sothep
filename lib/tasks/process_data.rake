namespace :process_data do


  desc 'Create border crossings'
  task :borders_crossing => :environment do

    borders_crossings_hash = {}
    borders = [
      [ :nashua, :dunwich ], [ :nashua, :lowell ],
      [ :plainfield, :oxford ], [ :plainfield, :providence ], [ :plainfield, :westerly ],
      [ :woonsocket, :milford ], [ :woonsocket, :dedham ], [ :woonsocket, :attleboro ], [ :woonsocket, :fall_river ], [ :woonsocket, :taunton ],
      [ :providence, :attleboro ], [ :providence, :fall_river ], [ :providence, :taunton ],
      [ :fall_river, :bristol ], [ :fall_river, :newport ]
    ]

    borders.each do |border|
      p [ border.first, border.last ]
      borders_crossings_hash[ border.first ] ||= []
      borders_crossings_hash[ border.first ] << border.last
      borders_crossings_hash[ border.last ] ||= []
      borders_crossings_hash[ border.last ] << border.first
    end

    File.open( 'app/models/game_core/map/data/borders_crossings.yml', 'w' ) do |f|
      f.print borders_crossings_hash.to_yaml
    end
  end

  desc 'Create destinations yaml file'
  task :destinations => :environment do

    destinations = File.open( 'work/destinations.json').read
    destinations = JSON.parse( destinations )

    final_destinations_hash = {}

    destinations.each do |k, v|
      final_destinations_hash[ k.to_sym ] ||= []
      final_destinations_hash[ k.to_sym ] += v.map{ |e| e.to_sym }
      final_destinations_hash[ k.to_sym ].uniq!
      v.each do |dest|
        final_destinations_hash[ dest.to_sym ] ||= []
        final_destinations_hash[ dest.to_sym ] << k.to_sym
        final_destinations_hash[ dest.to_sym ].uniq!
      end
    end

    File.open( 'app/models/game_core/map/data/destinations.yml', 'w' ) do |f|
      f.puts( final_destinations_hash.to_yaml )
    end

  end

  desc 'Print locations as yaml file'
  task :locations => :environment do
    locations = {}

    klass = :c
    [ 'work/cities.txt', 'work/water_areas.txt' ].each do |filename|
      File.open( filename, 'r' ) do |f|
        record = []
        f.readlines.each do |line|
          line = line.chomp

          if line.empty?
            locations[ record[0].to_sym ] = { klass: klass, port: record[3] == 'true', x: record[1].to_i, y: record[2].to_i }
            record = []
          else
            record << line
          end
        end
      end
      klass = :w
    end

    File.open( 'app/models/game_core/map/data/locations.yml', 'w' ) do |f|
      f.print locations.to_yaml
    end
  end

end