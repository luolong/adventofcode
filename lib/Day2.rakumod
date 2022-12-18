unit module Day2;

constant @Hand = <Rock Paper Scissors>;
constant @Result = <Lose Tie Win>;

subset Hand   of Str where * ~~ any @Hand;
subset Result of Str where * ~~ any @Result;

sub infix:<vs>(Hand:D $a, Hand:D $b --> Result) {
  given @Hand.first($a, :k) - @Hand.first($b, :k) {
    when 0 { 'Tie' }
    when 1 { 'Win' }
    default { 'Lose' }
  }
}

multi score(Hand:D $h)   { @Hand.first($h, :k) + 1 }
multi score(Result:D $r) { @Result.first($r, :k) * 3 }
multi score-with(Hand:D $a, Hand:D $b) { 
  score($b) + score($b vs $a); 
}

multi score-to(Hand:D $a, Result:D $r) {
  my $b = @Hand[(@Hand.first($a, :k) + @Result.first($r, :k) - 1) % 3];
  score($b) + score($r)
}

sub hand(Str:D $h --> Hand:D) { $h }
sub result(Str:D $r --> Result:D) { $r }

sub solution-one(IO::Handle:D $input) is export {
  constant %code = (<A B C> Z @Hand), (<X Y Z> Z @Hand);
  [+] $input.lines.map: -> $line {
    if so $line {
      my ($a, $b) = $line.split(' ').map(-> $s {%code{$s}}).map: &hand;
      score-with(hand($a), hand($b)) 
    }
  }
}

sub solution-two(IO::Handle:D $input) is export {
  constant %code = (<A B C> Z @Hand), (<X Y Z> Z @Result);
  [+] $input.lines.map: -> $line {
    if so $line {
      my ($a, $b) = $line.split(' ').map: -> $s {%code{$s}};
      score-to(hand($a), result($b))
    }
  }
}

