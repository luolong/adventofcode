unit module Day6;

sub run(Str:D $file) is export {
  my @grouped-answers = $file.IO.split("\n\n", :skip-empty).map(-> $a {
    $a.split("\n", :skip-empty)>>.comb>>.Set
  });

  say qq:to/END/;
  Day 6, Part 1:
    Total sum of Yes answer counts for all groups is {
      [+] @grouped-answers.map(-> @g { [∪] @g })>>.elems
    }

  Day 6, Part 2:
    Corrected total sum of Yes counts for all groups is {
      [+] @grouped-answers.map(-> @g { [∩] @g })>>.elems
    }
  END
}
