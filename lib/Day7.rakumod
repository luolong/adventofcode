unit module Day7;

grammar LuggageProcessingRules {
  rule TOP               { ^^ (<luggage-rule>)+ $$ }

  rule luggage-rule      { <outer-bag> <contained-bags>  '.' }

  rule outer-bag         { <bag-color> 'bags' }
  rule contained-bags    { 'contain' [ 'no' 'other' 'bags' | (<containment-rule>)+ % ',' ] }
  rule containment-rule  { <bag-count> <bag-color> [ 'bag' | 'bags' ] }

  rule bag-color         { <.word> ** 1..2 }
  rule bag-count         { \d+ }
  rule word              { \w+ }
}


sub day7(Str $file) is export {
  say "Day 7 coming up soon...";
  say LuggageProcessingRules.parse: $file.IO.slurp.trim-trailing;
}
