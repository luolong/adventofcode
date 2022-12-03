unit module Day1;

my method get-calories(Seq:D: --> Seq:D) {
  gather {
    for self -> $calories {
      state $total = 0;
      when so $calories { $total += $calories }
      default {
        take $total.clone;
        $total = 0;
      }
    }
  }
}

sub run(Str $file) is export {
  say qq:to/END/;
  Day 1, Part 1:
    The elf is carrying { $file.IO.lines.&get-calories.max } calories.
  Day 1, Part 2:
    Three elves are carrying total of { [+] $file.IO.lines.&get-calories.sort.tail(3) } calories.
  END
}

