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

  method eval(&callback, |c --> Int) {
    my $index = 0;
    my $accumulator = 0;

    while $index < @!instructions.elems {
       my ($op, $arg) = @!instructions[$index].kv;
       given $op {
         return $accumulator unless callback($index, $op, $arg);
         when 'acc' { $accumulator += $arg; proceed }
         when 'jmp' { $index += $arg; }
         default { $index++; }
       }
    }
  }

  method elems { @!instructions.elems }
}

sub run(Str $file) is export {
  say "Day 8, Part 1:";

  my $interpreter = Interpreter.new: input => $file.IO.slurp;
  my Bool @visited[$interpreter.elems];
  say "  Accumulator value before entering infinite loop is ", $interpreter.eval: sub ($i, $op, $arg) {
    return False if @visited[$i];
    @visited[$i] = True;
    True
  };
}
