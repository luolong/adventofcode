unit module Day1;

my method get-calories(Seq:D: --> Seq:D) {
  gather {
    for self -> $calories {
      state $total = 0;
      when so $calories { $total += $calories }
      default { take $total.clone; $total = 0; }
    }
  }
}

sub solution-one(IO::Handle:D $input) is export { $input.lines.&get-calories.max }
sub solution-two(IO::Handle:D $input) is export { [+] $input.lines.&get-calories.sort.tail(3) }

