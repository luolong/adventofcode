unit module Day1;


sub day1(Str $file, Bool :$verbose = False) is export {
  my method solution(Seq:D: $size) {
    with self.combinations($size).first({ .sum == 2020 }) {
      "{ .join: ' x ' } = { .reduce: &infix:<*> }"
    }
  }

  say "Day1, part1: ", $file.IO.lines.&solution: 2;
  say "Day1, part2: ", $file.IO.lines.&solution: 3;
}
