namespace :process_data do

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