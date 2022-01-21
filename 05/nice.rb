#!/usr/bin/env ruby
class String
  @@double = ('a'..'z').map { |c| "#{c}#{c}"}

  def double?
    for double in @@double
      if self.match(double); return true; end
    end

    false
  end

  def nice?
    self.double? and self.count('aeiou') >= 3 and not self.match?(/(ab)|(cd)|(pq)|(xy)/)
  end
end

def main
  fp = File.open 'input'

  cnice = 0
  for line in fp.readlines do
    if line.nice?; cnice += 1; end
  end

  puts("Nice String Counter: #{cnice}")
end

main()
