unit module Day4;

constant \MANDATORY = <byr iyr eyr hgt hcl ecl pid>;

proto is-valid(Str $name, $v --> Bool) { * }
multi is-valid('byr', Int(Str) $v) { return so $v ~~ 1920..2002 }
multi is-valid('iyr', Int(Str) $v) { return so $v ~~ 2010..2020 }
multi is-valid('eyr', Int(Str) $v) { return so $v ~~ 2020..2030 }
multi is-valid('hgt', Str $v) {
  given $v {
    when / 'cm' $ / { return so $v.substr(0, *-2).Int ~~ 150..193 }
    when / 'in' $ / { return so $v.substr(0, *-2).Int ~~  59..76  }
    default { return False }
  }
}
multi is-valid('hcl', $v) { return $v ~~ / ^ '#' <xdigit> ** 6 $ / }
multi is-valid('ecl', $v) { return $v ~~ any <amb blu brn gry grn hzl oth> }
multi is-valid('pid', $v) { return $v ~~ / ^ \d ** 9 $ / }


class Passport is Map {
  method parse(Str $_) {
    self.new( $_.trim.split(/<[ : \s ]>/) );
  }

  method all-fields-present {
    so self.{all(MANDATORY)}:exists
  }

  method all-fields-valid {
    my ($byr, $iyr, $eyr, $hgt, $hcl, $ecl, $pid) = self.{@(MANDATORY)};
    when so $byr & $iyr & $eyr & $hgt & $hcl & $ecl & $pid {
      return so is-valid('byr', $byr)
              & is-valid('iyr', $iyr)
              & is-valid('eyr', $eyr)
              & is-valid('hgt', $hgt)
              & is-valid('hcl', $hcl)
              & is-valid('ecl', $ecl)
              & is-valid('pid', $pid);
    }
  }
}


sub run(Str $file) is export {
  my @documents = $file.IO.open.split(/ \n\n /, :skip-empty)
                .map({ Passport.parse($_) });

  my Int $answer;

  say qq:to/END/;
  Day 4, Part 1:
    {
      @documents.grep({ $_.all-fields-present }).elems
    } valid passports (all mandatory fields present)

  Day 4, Part 2:
    {
      @documents.grep({ $_.all-fields-valid }).elems
    } valid passports (all mandatory fields are present and valid)
  END
}
