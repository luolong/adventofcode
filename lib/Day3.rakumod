unit module Day3;

constant $priorities = ('a'...'z', 'A'...'Z').join;

sub items-in-both-compartments(Str:D $s --> Set:D) {
  [(&)] $s.comb.batch($s.chars div 2)
}

sub solution-one(Seq:D $input) is export {
  [+] $input.grep({so $_})
            .map({items-in-both-compartments($_).keys.head})
            .map({$priorities.index($_) + 1});
}

sub solution-two(Seq:D $input) is export { "not implemented" }
