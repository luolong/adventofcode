unit module Day3;

constant $priorities = ('a'...'z', 'A'...'Z').join;

sub items-in-both-compartments(Str:D $s --> Set:D) {
  [(&)] $s.comb.batch($s.chars div 2)
}

sub solution-one(IO::Handle:D $input) is export {
  [+] $input.lines
            .grep({so $_})
            .map({items-in-both-compartments($_).keys.head})
            .map({$priorities.index($_) + 1})
}

sub solution-two(IO::Handle:D $input) is export {
  [+] $input.lines
            .grep({so $_})
            .batch(3)
            .map({([(&)] $_>>.comb).keys.head})
            .map({$priorities.index($_) + 1})
}
