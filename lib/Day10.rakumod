unit module Day10;

sub run(Str $file) is export {
  say "Day 10, Part 1";
  my @adapters = $file.IO.lines».Int;
  my @adapter-chain = sort flat (0, @adapters, @adapters.max + 3);

  my %buckets = @adapter-chain.rotor(2=>-1)
                              .classify({ [-] $_.reverse });

  my @elems = %buckets<1 3>».elems;
  say "  ", @elems.join(" * "), " = ", [*] @elems;

  say "Day 10, Part 2";
  say "  Adapters can be arranged in {count-variants @adapter-chain} ways";
}

sub count-variants(*@chain --> Int) {
  my @ways-to-reach of Int = flat (1, 0 xx @chain.elems - 1);
  for @chain.kv -> $i, $v {
    loop (my $j = $i + 1; $j < @chain && @chain[$j] - $v <= 3; $j++) {
      my $u = @ways-to-reach[$j];
      $u += @ways-to-reach[$i];
      @ways-to-reach[$j] = $u;
    }
  }
  return @ways-to-reach.tail;
}
