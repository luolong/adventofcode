#!/usr/bin/env just --justfile

mod run
mod test

# Print this helpful message
help:
  @just --list


# Just run today's solution
today:
  #!/usr/bin/env zsh
  case "$(date --iso-8601)" in
    2025-12-*)
      day=$(date +%d)
      just day "$day"
      ;;
    *)
      echo "This is not an Advent of Code. Use './just day <number>' instead."
      ;;
  esac

# Run solution for day #
@day number:
  just new {{number}}
  scripts/run.sh day {{number}}

# Init solution for a new day
new number:
  @scripts/new_day.sh {{number}}
