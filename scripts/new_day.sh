#!/usr/bin/env zsh
new_day() {
  if [[ -z "$1" ]]; then
    echo "Usage: just new <1..24>"
    exit 1
  fi

  DAY_NUMBER=$(($1))
  if [[ "${DAY_NUMBER}" -lt 1 || "${DAY_NUMBER}" -gt 24 ]]; then
    echo "Cannot create day #${DAY_NUMBER}"
    exit 1
  fi

  DAY_CLASS="Day${(l:2::0:)DAY_NUMBER}"
  DAY_PACKAGE="solution"
  DAY_FILENAME="src/${DAY_PACKAGE}/${DAY_CLASS}.java"

  if [[ -f "${DAY_FILENAME}" ]]; then
    sed -i "s|null, //new ${DAY_CLASS}(),|new ${DAY_CLASS}(),|" src/Main.java
    return 0
  fi

  export DAY_NUMBER DAY_CLASS DAY_PACKAGE
  cat <<EOF > "${DAY_FILENAME}"
package ${DAY_PACKAGE};

import java.util.*;
import java.lang.annotation.*;

/**
 * Solutions for Advent of Code 2025 Day ${DAY_NUMBER}
 */
public class ${DAY_CLASS} implements Part1, Part2 {
    //@Override
    //public String part1(String input) {
    //    return "Not implemented";
    //}

    //@Override
    //public String part2(String input) {
    //    return "Not implemented";
    //}
}
EOF

  sed -i "s|null, //new ${DAY_CLASS}(),|new ${DAY_CLASS}(),|" src/Main.java
}

# execute if not included as source
if [[ "${ZSH_EVAL_CONTEXT}" == "toplevel" ]]; then
  new_day "$@"
fi
