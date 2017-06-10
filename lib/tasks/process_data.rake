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

    File.open( 'app/models/game_core/map/borders_crossings.yml', 'w' ) do |f|
      f.print borders_crossings_hash.to_yaml
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

    File.open( 'app/models/game_core/map/locations.yml', 'w' ) do |f|
      f.print locations.to_yaml
    end
  end

end