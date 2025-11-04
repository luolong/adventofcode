#!/usr/bin/env zsh

compile() {
  sources=($(find src/ -name '*.java'))
  recompile=false
  for src in "${sources[@]}"; do
    if [[ ! -f bin/.timestamp || $src -nt bin/.timestamp ]]; then
      echo "Recompiling because $src is newer than bin/.timestamp"
      mkdir -p bin/
      date --iso-8601=seconds > bin/.timestamp
      javac --source 26 --target 26 \
            --enable-preview \
            --module-path src/ \
            -d bin/ \
            src/**/*.java
      s=$?
      return $s
    fi
  done
}

run() {
  java_args=
  java --enable-preview -ea \
       -XX:+UseCompactObjectHeaders \
       --class-path 'bin/' \
       Main "${@}" 2>/dev/null
}

day() {
  compile && run "${@}"
}

usage() {
  echo "Usage: run.sh [options] [all|day <number...>]" >&2
  echo "Options:"
  echo "  -t, --test      Run tests"
  echo "  -h, --help      Show this help message"
}

main() {
  if [[ $# -eq 0 ]]; then
    usage
    return 1
  fi

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --t|--test)
      arguments+=("--test")
      shift
    ;;
    --h|--help)
      echo "Usage: run.sh [options] [all|day <number...>]" >&2
      usage
      return 0
    ;;
    --*)
      echo "Invalid option: $1" >&2
      usage
      return 1
    ;;
    *)
      break
    ;;
    esac
  done

  cmd=$1; shift

  case "$cmd" in
    all)
      echo "Running solutions for all days"
      case "$(date --iso-8601)" in
        2025-12-*)
          today=$(date +%d)
          if [[ $today -gt 12 ]]; then
            day ${arguments} {1..12}
          else
            day ${arguments} {1..$(($(date +%d)))}
          fi
          ;;
        *)
          day ${arguments} {1..12}
          ;;
      esac
      ;;

    "day"|"days")
      day ${arguments} "${@}"
      ;;

    *)
      echo "Invalid command: $cmd" >&2
      echo "Usage: run.sh [all|day <number...>]" >&2
      return 1
      ;;
  esac
}


# execute if not included as source
if [[ "${ZSH_EVAL_CONTEXT}" == "toplevel" ]]; then
  main "${@}"
fi
