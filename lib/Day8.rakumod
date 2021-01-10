unit module Day8;

#use Grammar::Tracer;
grammar BootCode {
  rule TOP                       { ^ <instruction>+ %% <.lf> $ }

  rule instruction               { <operation> <argument> }

  proto token operation          {*}
        token operation:sym<acc> { <sym> }
        token operation:sym<jmp> { <sym> }
        token operation:sym<nop> { <sym> }

  token argument                 { <[ + - ]> \d+ }

  token lf { \v+ }
  token ws { \h* }
}

class Interpreter does Callable {
  has @!instructions;

  submethod BUILD(Str :$input) {
    @!instructions = BootCode.parse($input, actions => class {
      method TOP($/)         { make $<instruction>».made }
      method instruction($/) { make $<operation>.Str => $<argument>.made }
      method argument($/)    { make $/.Int }
    }).made or die "Invalid boot code!}";
  }

  method exec(&callback) {
    my $index = 0;
    my $accumulator = 0;

    while $index < @!instructions.elems {
       my ($op, $arg) = callback($index, |@!instructions[$index].kv);
       without $op { return $accumulator, False };
       given $op {
         when 'acc' { $accumulator += $arg; proceed }
         when 'jmp' { $index += $arg; }
         default { $index++; }
       }
    }

    return $accumulator, True;
  }

  method elems { @!instructions.elems }
}

sub run(Str $file) is export {
  say "Reading input from $file";
  my $interpreter = Interpreter.new: input => $file.IO.slurp;

  my Int  @indices is Array = Empty;
  my %replace = Map.new:
    'jmp' => 'nop',
    'nop' => 'jmp';
  
  say "Day 8, Part 1:";
  do {
    my Bool @visited[$interpreter.elems];
    my ($acc1, $) = $interpreter.exec: sub ($i, $op, $arg) {
      return Nil if @visited[$i];
      @indices.push: $i when $op eq any %replace.keys;
      @visited[$i] = True;
      $op, $arg
    };
    say "  Accumulator value before entering infinite loop is $acc1";
  }

  
  say "Day 8, Part 2";
  my ($acc2, $) = @indices.race.map(-> $index {
    my Bool @visited[$interpreter.elems];
    my ($acc, $done) = $interpreter.exec: sub ($i, $op, $arg) {
      return Nil if @visited[$i];
      @visited[$i] = True;
      $i == $index ?? (%replace{$op}, $arg) !! ($op, $arg);
    };
  })
  .first({ so $_[1] });
  say "  Accumulator value after successful boot sequence is $acc2";
}

run("day8.txt".IO.resolve.path)