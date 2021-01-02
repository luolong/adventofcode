unit module Day7;

grammar LuggageProcessingSpec {
  rule TOP               { ^^ <luggage-rule>+ $$ }

  rule luggage-rule      { <outer-bag> <contained-bags>  '.' }

  rule outer-bag         { <bag-color> 'bags' }
  rule contained-bags    { 'contain' [ 'no' 'other' 'bags' | <containment-rule>+ % ',' ] }
  rule containment-rule  { <bag-count> <bag-color> [ 'bag' | 'bags' ] }

  rule bag-color         { <.word> ** 1..2 }
  rule bag-count         { \d+ }
  rule word              { \w+ }
}

class LuggageProcessingSpec-actions {
  method TOP($/)              { make $<luggage-rule>».made; }
  method luggage-rule($/)     { make $<outer-bag>.made => $<contained-bags>.made }

  method outer-bag($/)        { make $<bag-color>.made }
  method contained-bags($/)   { make $<containment-rule>».made.Bag }
  method containment-rule($/) { make $<bag-color>.made => $<bag-count>.made; }

  method bag-color($/)        { make $/.Str.trim }
  method bag-count($/)        { make $/.Int }
}

my method count-bags(Map:D: Str $color) {
  1 + ([+] self{$color}.map({ .value * self.&count-bags(.key) }))
}

constant $SHINY_GOLD = "shiny gold";
sub day7(Str $file) is export {

  my $rules = LuggageProcessingSpec.parse(
      $file.IO.slurp.trim-trailing,
      actions => LuggageProcessingSpec-actions,
      ).made;

  say "Day 7, Part 1:";
  my %s  = ∅.SetHash;
  my @pool = $rules.grep({ $SHINY_GOLD ∈ $_.value })».key;
  while @pool {
    my $bag = pop @pool;
    %s ∪= $bag;
    @pool.prepend: $rules.grep({ $bag ∈ $_.value })».key;
  }
  say "  { %s.keys.elems } bag colors can eventually contain at least one shiny gold bag.";

  say "Day 7, Part 1:";
  my $bags = $rules.Map.&count-bags($SHINY_GOLD) - 1;
  say "  $bags individual bags are required inside your single $SHINY_GOLD bag."
}
