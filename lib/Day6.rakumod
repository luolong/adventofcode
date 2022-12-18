unit module Day6;

sub solution-one(IO::Handle:D $input) is export {
  my Int $i = 4;
  for $input.comb.rotor(4=>-3) {
    return $i if .Bag.elems == 4;
    $i++
  }
  Nil
}

sub solution-two(IO::Handle:D $input) is export {
  my Int $i = 14;
  for $input.comb.rotor(14=>-13) {
    return $i if .Bag.elems == 14;
    $i++
  }
  Nil
}
