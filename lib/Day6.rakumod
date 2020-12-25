unit module Day6;

sub day6(Str:D $file) is export {
  my @grouped-answers = $file.IO.split("\n\n", :skip-empty).map(-> $a {
    $a.split("\n", :skip-empty)>>.comb>>.Set
  });

  say "Day 6, Part 1:";
  say "  Total sum of Yes answer counts for all groups is ",
      [+] @grouped-answers.map(-> @g { [∪] @g })>>.elems;

  say "Day 6, Part 2:";
  say "  Corrected total sum of Yes counts for all groups is ",
      [+] @grouped-answers.map(-> @g { [∩] @g })>>.elems;
}
