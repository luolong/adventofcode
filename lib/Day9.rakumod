
unit module Day9;

sub find-first-invalid(@lines, Int $preamble-size=25 --> Pair) {
  for @lines[$preamble-size..^*].kv -> $k, $v {
    my @preamble = @lines[($k..^*+$preamble-size).($k)];
    return $k => $v unless so $v == any @preamble.combinations(2).map({[+] $_});
  }
}

sub find-weakness(@lines, Int $index, Int $value --> Int) {
  my ($i1, $i2) = (0, 1);
  while $i2 < $index {
    my @slice = @lines[$i1..$i2];
    my $sum = [+] @slice;
    given $sum {
      when $value {
        my ($min, $max) = ( @slice.min, @slice.max );
        return $min + $max;
      }
      when $_ < $value { $i2 += 1; }
      when $_ > $value { $i1 += 1; }
    }

    $i2 = $i1 + 1 unless $i1 < $i2;
  }
}

sub run(Str $file) is export {
  say "Day 9, Part 1";
  my @lines = $file.IO.lines>>.Int;
  my ($index, $value) = find-first-invalid(@lines).kv;
  say "  First invalid value is $value";

  say "Day 9, Part 2";
  my $weakness = find-weakness(@lines, $index, $value);
  say "  Encryption weaknes is $weakness";
}
