unit module Day5;

sub day5(Str:D $file) is export {
  my @seat-ids = $file.IO.lines
                .grep({ .Bool })
                .map(&to-pair)
                .map(&to-seat-id)
                .sort;

  say "Day 5, Part 1: Max seat ID is ", @seat-ids.max;
  say "Day 5, Part 2: Your seat ID is ", @seat-ids.rotor(2 => -1)
     .first({ ([-] $_.reverse) == 2 })
     .min.succ;
}

sub to-pair(Str $input where / ^^ <[FB]> ** 7  <[LR]> ** 3 $$ / --> List:D) {
  $input.substr(0, 7).&to-int(<F B>), $input.substr(7, *).&to-int(<L R>)
}

sub to-seat-id(*@list --> Int) {
  [+] @list Z* (8, 1)
}

my method to-int(Str:D: @bits --> Int) {
  my $str = self;
  for @bits.kv -> $i, $bit {
    $str ~~ s :g /$bit/$i/
  }
  return ":{@bits.elems}<$str>".Int;
}
