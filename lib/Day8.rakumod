unit module Day8;

sub solution-one(IO::Handle:D $input) is export {
  my @map = $input.lines>>.comb>>.Int;

  my ($m, $n) = +@map, +@map.head;
  my @visible = [False xx $m] xx $n;
  
  my @i = cache ^$m X ^$n;
  for [@i, [@i.reverse]] -> @indices {
    my ($mh, $mv) = [-1 xx $m], [-1 xx $n];
    for @indices -> [$h, $v] {
      my $tree = @map[$h][$v];
      if $mh[$h] < $tree { $mh[$h] = $tree; @visible[$h][$v] = True }
      if $mv[$v] < $tree { $mv[$v] = $tree; @visible[$h][$v] = True }
    }
  }

  [+] @visible>>.sum
}


sub count-trees(*@trees --> Int) {
  my $i = @trees.grep(* >= @trees.first, :k)[1];
  return @trees - 1 without $i;
  $i
}

sub solution-two(IO::Handle:D $input) is export {
  my @rows = $input.lines>>.comb>>.Int; 
  my @cols = [Z] @rows; # Transpose list of lists
  my ($m, $n) = +@rows, +@rows.head; # Dimensions

  max [^$m X ^$n].race.map: -> [$r, $c] {
    my %rc = row=>@rows[$r].flat, col=>@cols[$c].flat;

    [*] (
      count-trees(%rc<col>[^$r.succ].reverse),
      count-trees(%rc<row>[^$c.succ].reverse),
      count-trees(%rc<col>[$r..*]),
      count-trees(%rc<row>[$c..*])
    )
  }
}
