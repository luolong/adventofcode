unit module Day5;

grammar BoardingPass {
  has Int $.row;
  has Int $.column;

  method parse(Str $input --> BoardingPass) {
    return BoardingPass.new(
      row => $input.substr(0, 7).&convert(<F B>),
      column => $input.substr(7, *).&convert(<L R>)
    )
  }

  method seat-id { 8 * $.row + $.column }

  multi method gist(BoardingPass:D:) {
    'Row ' ~ $.row ~ ', Column ' ~ $.column ~ ', seat ID ' ~ $.seat-id
  }
}

my method convert(Str:D: @bits) {
  my $str = self;
  for @bits.kv -> $i, $bit {
    $str ~~ s :g /$bit/$i/
  }
  return ":{@bits.elems}<$str>".Int;
}

sub day5(Str:D $file) is export {
  my @boarding-passes =  $file.IO.lines.grep({ .Bool }).map({
     BoardingPass.parse($_)
   });

  say "Day 5, Part 1: Max sead ID is ", @boarding-passes.map({ .seat-id }).max;

  .say for @boarding-passes.sort({ $^a.seat-id cmp $^b.seat-id }).rotor(3 => -2).grep( -> @list {
    my $offset = @list[0].seat-id;
    my @l = @list>>.seat-id Z- $offset xx *;
    not (@l eqv [0, 1, 2])
  });
}
