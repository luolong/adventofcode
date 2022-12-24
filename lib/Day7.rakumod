unit module Day7;

grammar Filesystem {
    token TOP   { [ <input> | <output> | <.ws> ] * %% <.lf> }
    rule input  { '$' <command> }
    rule output { <dir> | <file> }

    proto token command         {*}
          token command:sym<cd> { <sym> <.ws> <arg> }
          token command:sym<ls> { <sym> }
    
    proto token arg             {*}
          token arg:sym<root>   { '/' }
          token arg:sym<parent> { '..' }
          token arg:sym<*>      { <name> }
    
    rule dir   { dir <name> }
    rule file  { <size> <name> }
    
    token name { \S+ }
    token size { \d+ }

    token lf  { \n }
    token ws  { \h* }
}

class Directories {
  has     %!dirs is built(False) = { '/' => 0 };
  has Str $!cwd  is built(False) = '';

  method name($/) { make $/.Str }
  method size($/) { make $/.Int }
  
  method full-path(Str $name)   { $!cwd.subst: / \/? $/, "/$name" }
  method base-name() { my $b = $!cwd.subst(/ \/ <-[/]>+ $/, ''); $b or '/' }

  method arg:sym<root>($/)   { make '/' }
  method arg:sym<parent>($/) { make self.base-name }
  method arg:sym<*>($/)      { make self.full-path: $<name>.made }

  method command:sym<cd>($/) { $!cwd = $<arg>.made }

  method dir($/)  { my $name = self.full-path($<name>.made); %!dirs{$name} = 0 }
  
  method file($/) {
    my ($name, $size) = ($!cwd, $<size>.made);
    for %!dirs.keys.grep({$name.starts-with: $_}) { 
      %!dirs{$_} += $size
    }
  }

  method TOP($/) { make {sizes=>%!dirs.values, free=>70000000 - %!dirs</> } }
}

sub solution-one(IO::Handle:D $input) is export {
  [+] Filesystem.parse($input.slurp, actions => Directories.new).made<sizes>.grep({$_ <= 100000})
}

sub solution-two(IO::Handle:D $input) is export {
  my %fs = Filesystem.parse($input.slurp, actions => Directories.new).made;
  %fs<sizes>.grep({$_ >= (30000000 - %fs<free>)}).min
}
