module GameLogic::BorderCrossing

  def check_cross_border( current_loc, dest_loc, investigator )
    if dest_loc.class == CCity && current_loc.class == CCity
      road = current_loc.outgoing_r_roads.where( dest_city_id: dest_loc.id )
      raise "More than one road : #{road.inspect}" if road.count > 1

      road = road.first
      puts "Src city : #{road.src_city.inspect}"
      puts "Dest city : #{road.dest_city.inspect}"
      puts "Road : #{road.inspect}"

      if road.border
        event = "#{investigator.code_name.humanize} cross a border "

        dice = rand( 1 .. 6 )
        event << 'and get no problem' if dice >= 3
        event << 'and get problem with the border police' if dice <= 2

        EEventLog.create!( event: event )
      end
    end
  end

end