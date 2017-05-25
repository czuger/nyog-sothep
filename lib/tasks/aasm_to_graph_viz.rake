desc 'Print investigator graph'
task :print_investigator_graph => :environment do


  def print_current_state_transitions_and_deep( _object, states_done, transitions )

    _object.aasm.states(:permitted => true).map(&:name).each do |sub_state|
      # print sub_state.inspect
      transitions << "#{_object.aasm_state} -> #{sub_state}"
    end

    next_event = ( _object.aasm.events(:permitted => true).map(&:name) - states_done ).first
    if next_event

      states_done << next_event
      _object.send( next_event )

      print_current_state_transitions_and_deep( _object, states_done, transitions )
    end

  end

  i = IInvestigator.new
  states_done = []
  transitions = []
  out_file = File.open( 'out.gviz', 'w' )

  out_file.puts 'digraph G {'

  print_current_state_transitions_and_deep( i, states_done, transitions )

  p transitions
  transitions.uniq!
  p transitions

  out_file.puts transitions.join( "\n" )

  out_file.puts '}'

  `dot out.gviz -Tpng > out.png`

  # def print_states
  #   self.aasm.states.map(&:name).reject{ |s| s == :run }
  # end
  #
  # def self.print_graph_in_graph_viz_state( start_state = :move, current_state = nil )
  #   unless current_state
  #     @aasm_graph_states = IInvestigator.aasm.states.map(&:name).reject{ |s| s == start_state }
  #     @aasm_graph_transitions = []
  #     current_state = start_state
  #
  #     job.aasm.states(:permitted => true).map(&:name).each do |sub_state|
  #       @aasm_graph_transitions << "#{current_state} -> #{sub_state}"
  #     end
  #     job.aasm.states(:permitted => true).map(&:name).each do |sub_state|
  #       @aasm_graph_transitions << "#{current_state} -> #{sub_state}"
  #     end
  #
  #   end
  # end
  #
  # def print_current_state_transitions( current_state = nil )
  #
  #   self.aasm_state = current_state
  #
  #   aasm.states(:permitted => true).map(&:name).each do |sub_state|
  #     print sub_state.inspect
  #     # print "#{current_state} -> #{sub_state}"
  #   end
  #
  #   aasm.states(:permitted => true).map(&:name)
  #
  # end

  # File.open( 'work/water_areas_links.txt', 'r' ) do |f|
  #   record = []
  #   f.readlines.each do |line|
  #     line = line.chomp
  #     if line.empty?
  #       puts "About to create : #{record.inspect}"
  #
  #       source_water_area = WWaterArea.find_by( code_name: record[0] )
  #       raise "Unknown water area for #{record[0]}" unless source_water_area
  #       record.shift
  #
  #       record.each do |line|
  #         c = CCity.find_by( code_name: line )
  #         unless c
  #           w = WWaterArea.find_by( code_name: line )
  #           puts "About to insert : #{w.inspect}"
  #           if w
  #             source_water_area.connected_w_water_areas << w unless source_water_area.connected_w_water_areas.exists?( w.id )
  #             w.connected_w_water_areas << source_water_area unless w.connected_w_water_areas.exists?( source_water_area.id )
  #           end
  #         else
  #           source_water_area.ports << c
  #         end
  #       end
  #
  #       record = []
  #     else
  #       record << line
  #     end
  #   end
  # end

end
