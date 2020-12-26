unit module Day2;

grammar Line {
  token TOP      { ^ <low=num> '-' <high=num> <.ws> <char> ':' <.ws> <password> $ }

  token num       { \d+ }
  token char      { <alpha> }
  token password  { .* }
}

class CountChars {
  method TOP($/) {
    make $<password>.Str.indices($<char>).elems ~~ ($<low>..$<high>);
  }
}

class CheckIndices {
  method TOP($/) {
    my $indices = set $<low>.Str.Int - 1, $<high>.Str.Int - 1;
    make ($<password>.Str.indices($<char>).Set (&) $indices).elems == 1;
  }
}

sub day2(Str $file) is export {
  my @lines = $file.IO.lines;
  for CountChars, CheckIndices {
    my $count = [+] @lines.map(-> $line { Line.parse($line, actions => $_).made });

    state $part = 1;
    say qq:to/END/;
    Day 2, Part { $part++ }:
      $count valid passwords";
    END
  }
}
