
unit module Day10;

sub run(Str $file) is export {
  say "Day 10, Part 1";
  my @adapters = $file.IO.lines>>.Int;
  #my @adapters = <16 10 15 5 1 11 7 19 6 12 4>;
  my @adapter-chain = sort flat (0, @adapters, @adapters.max + 3);

  my %buckets = @adapter-chain
    .rotor(2=>-1)
    .classify({ [-] $_.reverse });

  my @elems = %buckets<1 3>>>.elems;
  say "  ", @elems.join(" * "), " = ", [*] @elems
}
