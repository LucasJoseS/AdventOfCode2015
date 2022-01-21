#!/usr/bin/env ruby
class String
  @@double = ('a'..'z').map { |c| "#{c}#{c}" }
  @@middle = ('a'..'z').map { |c| /#{c}\w#{c}/ }

  def double?
    for double in @@double
      if self.match(double); return true; end
    end
    false
  end

  def nice?
    self.double? and self.count('aeiou') >= 3 and not self.match?(/(ab)|(cd)|(pq)|(xy)/)
  end

  def middle?
    @@middle.each { |m| if m.match(self); return true; end }
    false
  end
  
  def pair?
    0.upto self.length-1 do |index|
      if /#{self[index]}#{self[index+1]}.*#{self[index]}#{self[index+1]}/.match?(self); return true; end
    end

    false
  end

  def better_nice?
    self.middle? and self.pair?
  end
end

def main
  fp = File.open 'input'

  cnice = 0; bcnice = 0
  for line in fp.readlines do
    if line.nice?; cnice += 1; end
    if line.better_nice?; bcnice += 1; end
  end

  puts("Nice String Counter: #{cnice}")
  puts("Better Nice String Counter: #{bcnice}")
end

main()
