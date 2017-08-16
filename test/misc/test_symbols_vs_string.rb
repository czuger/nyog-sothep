require 'benchmark'

string_array = []
symbols_array = []

1.upto(50000).each do
  string_array << rand( 1 .. 100000 ).to_s
  symbols_array << rand( 1 .. 100000 ).to_s.to_sym
end

# Finding in strings
rt = Benchmark.realtime{
  elem = rand( 1 .. 100000 ).to_s
  1.upto(1000).each do
    _ = string_array.include?( elem )
  end
}

puts "Found in strings in #{rt*1000} ms"

# Finding in symbols
rt = Benchmark.realtime{
  elem = rand( 1 .. 100000 ).to_s.to_sym
  1.upto(1000).each do
    _ = symbols_array.include?( elem )
  end
}

puts "Found in strings in #{rt*1000} ms"