unit module Day7;

use Grammar::Tracer;

class Fs is Associative {
  has     %!fs   is built(False) handles('sort') = '/' => { type => 'dir', size => 0 };
  has Str $!cwd  is built(False);

  our sub load-from-input(@lines --> Fs:D) {
    my %fs := Fs.new;
    for @lines {
      when m/^ '$' <.ws> cd <.ws> $<name> = [\S+] /    { %fs.cd: $<name>.Str }
      when m/^ '$' <.ws> ls /                          {  }
      when m/^ dir <.ws> $<name> = [\S+] /             { %fs.ls-dir: $<name>.Str }
      when m/^ $<size> = [\S+] <.ws> $<name> = [\S+] / { %fs.ls-file: $<size>.Int, $<name>.Str }
      default { * }
    }
    %fs
  }

  method cd(Str:D $path) {
    given $path {
      when '/'  { $!cwd = '/' }
      when '..' { $!cwd =  $!cwd.subst(/\/<-[/]>+\/?$/, '/') }
      default   { $!cwd ~= "{$path}/" }
    }
  }

  method ls-dir(Str $name) {
    my $path = $!cwd ~ "{$name}/";
    if not %!fs{$path}:exists {
      %!fs{$path} = { type => 'dir', size => 0 }
    }
  }

  method ls-file(Int $size, Str $name) {
    my $parent = $!cwd;
    my $path = $parent ~ "{$name}";
    %!fs{$path} = { type=>'file', size=>$size };
    for (%!fs<>:k).grep({.ends-with('/') and $path.starts-with($_)}) {
      %!fs{$_}<size> += $size
    }
  }

  method free() {
    70000000 - %!fs</><size>
  }

  method gist(Fs:D: --> Str) {
    %!fs.sort.map({ 
      my @path = $_.key.split('/', :skip-empty);
      "{'  ' x @path.elems}- {@path.tail // '/'} ({$_.value<type>}, size={$_.value<size>})"
    }).join("\n")
  }
}


sub solution-one(IO::Handle:D $input) is export {
  my %fs := Fs::load-from-input($input.lines);
  [+] %fs.sort>>.value.grep({ $_<type> ~~ 'dir' and $_<size> <= 100000 })>><size>
}

sub solution-two(IO::Handle:D $input) is export {
  my %fs := Fs::load-from-input($input.lines);
  %fs.sort>>.value.grep({ $_<type> ~~ 'dir' and $_<size> >= (30000000 - %fs.free) })>><size>.min
}
