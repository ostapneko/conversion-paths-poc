(1..100000).each do
  line = (1..100).map do
    rand(100)
  end.join(',')

  puts line
end
