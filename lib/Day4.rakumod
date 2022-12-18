unit module Day4;

sub range(Int $a, Int $b --> Range) { $a..$b }

sub solution-one(IO::Handle:D $input) is export {
  [+] $input.lines
            .map({ .split(/<[,-]>/)>>.Int.batch(2).map({$_[0]..$_[1]}) })
            .map({ [⊆] $_.cache or [⊇] $_ })
}

sub solution-two(IO::Handle:D $input) is export {
  [+] $input.lines
            .map({ .split(/<[,-]>/)>>.Int.batch(2).map({$_[0]..$_[1]}) })
            .map({ so ([∩] $_) })
}
