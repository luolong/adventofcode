unit module Day5;

sub simulate-crane-operator(IO::Handle:D $input, &crane-model:(%stacks, :$move, :$from, :$to)) {
  my %stacks;

  my Str $line = '';
  repeat {
    $line = $input.get;
    for $line.comb(4)>>.trim>>.&{ m/<[A..Z]>/ ?? $/.Str !! '' }.pairs.grep({.value})>>.kv -> ($to, $crate) {
      my $key = $to.succ;
      my $stack = %stacks{$key} // Array;
      $stack.prepend: $crate;
      %stacks{$key} = $stack;
    }
  } while so $line;

  for $input.lines.grep({ $_ }) -> $line {
    &crane-model(%stacks, |$line.words.Hash);
  }

  %stacks.sort.map({ .value.tail }).join
}

sub cratemover9000(%stacks, :$move, :$from, :$to) {
  for 1...$move { %stacks{$to}.push: %stacks{$from}.pop }
}

sub cratemover9001(%stacks, :$move, :$from, :$to) {
    %stacks{$to}.append: %stacks{$from}.splice: * - $move;
}

sub solution-one(IO::Handle:D $input) is export {
  simulate-crane-operator($input, &cratemover9000)
}

sub solution-two(IO::Handle:D $input) is export {
  simulate-crane-operator($input, &cratemover9001)
}
