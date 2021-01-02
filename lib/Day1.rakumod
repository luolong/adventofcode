unit module Day1;

sub day1(Str $file) is export {
  my method solution(Seq:D: $size) {
    with self.combinations($size).first({ .sum == 2020 }) {
      "{ .join: ' x ' } = { .reduce: &infix:<*> }"
    }
  }

  for 1..2 -> $i {
    say qq:to/END/;
    Day 1, Part $i:
      { $file.IO.lines.&solution: $i+1 }
    END
  }

}
