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
  say "Day 2, part 1: ", ([+] $file.IO.lines.map({ Line.parse($_, actions => CountChars).made })), " valid passwords";
  say "Day 2, part 2: ", ([+] $file.IO.lines.map({ Line.parse($_, actions => CheckIndices).made })), " valid passwords";
}
