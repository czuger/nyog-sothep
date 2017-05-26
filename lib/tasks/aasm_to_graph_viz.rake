desc 'Print investigator graph'
task :print_investigator_graph => :environment do


  def print_current_state_transitions_and_deep( _object, states, transitions, events )

    states.each do |state|
      _object.aasm_state = state

      _object.aasm.events(:permitted => true).map(&:name).each do |event_name|
        events << event_name
        _object.aasm_state = state

        transitions << "#{_object.aasm_state} -> #{event_name}"
        _object.send( event_name )
        transitions << "#{event_name} -> #{_object.aasm_state}"
      end
    end
  end

  states = IInvestigator.aasm.states.map(&:name)
  i = IInvestigator.new

  transitions = []
  events = []
  out_file = File.open( 'out.gviz', 'w' )

  out_file.puts 'digraph G {'

  print_current_state_transitions_and_deep( i,  states, transitions, events )

  transitions.uniq!
  events.uniq!

  events.each do |state|
    out_file.puts "#{state} [shape=box];"
  end

  out_file.puts transitions.join( "\n" )

  out_file.puts '}'

  p system( '/usr/bin/dot out.gviz -Tpng -o out.png' )

end
