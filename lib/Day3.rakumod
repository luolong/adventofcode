unit module Day3;

sub infix:<count-trees>(@arr, @xy --> Int) {
  (@xy, { @(.[] Z+ @xy) } ...^ *.[1] >= @arr)
    .grep({ @arr[.[1] ; .[0] % * ] eq '#' })
    .elems
}

sub day3(Str $file) is export {
  my @map = $file.IO.lines>>.comb;

  say qq:to/END/;
  Day 3, Part 1:
    { @map count-trees (3, 1) } trees";

  Day 3, Part 2:
    { [*] gather {
      for  (1, 1; 3, 1; 5, 1; 7, 1; 1, 2) -> @slope {
        take @map count-trees @slope
      }
    } }
  END
}
