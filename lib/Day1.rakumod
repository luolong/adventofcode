unit module Day1;

sub day1(Str $file, Int $count = 2) is export {
    for $file.IO.lines.combinations($count).grep({ $_.sum == 2020 }) -> $pair {
        say "{ $pair.join: ' ⨉ ' } = { $pair.reduce: &infix:<*> }";
    }

}
