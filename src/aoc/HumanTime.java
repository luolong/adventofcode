package aoc;

import java.time.Duration;
import java.util.StringJoiner;
import java.util.concurrent.TimeUnit;

public final class HumanTime {

  private final long totalNanos;

  public HumanTime(long totalnanos) {
    this.totalNanos = totalnanos;
  }

  public Duration getDuration() {
    return Duration.ofNanos(totalNanos);
  }

  public String toString() {
    StringJoiner out = new StringJoiner(" ");
    long nanos = totalNanos;
    long seconds = TimeUnit.NANOSECONDS.toSeconds(nanos);
    if (seconds > 0) {
      nanos -= -TimeUnit.SECONDS.toNanos(seconds);

      var s = seconds % 60;
      var minutes = seconds / 60;
      if (minutes > 0) {
        out.add("%d min".formatted(minutes));
      }

      out.add("%d s".formatted(s));
    }

    if (nanos > 10000L) {
      double millis = (double) nanos / 1000000.00;
      out.add("%.3fms".formatted(millis));
    } else {
      out.add("%dns".formatted(nanos));
    }

    return out.toString();
  }
}
