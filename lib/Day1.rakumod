unit module Day1;

sub part1(Seq $input) is export {
  return count-calories($input).max;
}

sub part2(Seq $input) is export {
  return [+] count-calories($input).sort.tail(3);
}

sub count-calories(Seq $input) {
  my @all-calories = Array[Int].new(0);

  for $input.Seq -> $v {
    when so $v { @all-calories.tail += $v.Int }
    default { @all-calories.push(0) }
  }

  return @all-calories;
}
