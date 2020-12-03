unit module Day1;


sub day1(Str $file) is export {
  my method solution(Seq:D: $size) {
    self.combinations($size).grep({ $_.sum == 2020 }).map(-> $p {
      "{ $p.join: ' x ' } = { $p.reduce: &infix:<*> }"
    })
  }

  say "Day1, part1: ", $file.IO.lines.&solution: 2;
  say "Day1, part2: ", $file.IO.lines.&solution: 3;
}
