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

sub solution-one(Seq:D $input) is export { $input.&get-calories.max }
sub solution-two(Seq:D $input) is export { [+] $input.&get-calories.sort.tail(3) }

