unit module Day4;

sub range(Int $a, Int $b --> Range) { $a..$b }

sub solution-one(Seq:D $input) is export {
  [+] $input.map({ .split(/<[,-]>/)>>.Int.batch(2).map({$_[0]..$_[1]}) })
            .map({ [⊆] $_.cache or [⊇] $_ })
}

sub solution-two(Seq:D $input) is export {
  [+] $input.map({ .split(/<[,-]>/)>>.Int.batch(2).map({$_[0]..$_[1]}) })
            .map({ so ([∩] $_) })
}
