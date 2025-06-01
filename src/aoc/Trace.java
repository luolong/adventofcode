package aoc;

import static aoc.Trace.Part.Part1;
import static aoc.Trace.Part.Part2;

import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.concurrent.ConcurrentLinkedQueue;
import jdk.jfr.Category;
import jdk.jfr.Description;
import jdk.jfr.Event;
import jdk.jfr.Label;
import jdk.jfr.Name;
import jdk.jfr.StackTrace;
import jdk.jfr.consumer.RecordedEvent;
import jdk.jfr.consumer.RecordingStream;

public class Trace implements AutoCloseable {

  private static final String AOC = "AOC";

  public enum Part {
    Part1,
    Part2;

    public static int count() {
      return Part.values().length;
    }

    @Override
    public String toString() {
      return "Part " + (ordinal() + 1);
    }

    public boolean isSet(int part) {
      return part == ordinal() + 1 || part == -1;
    }

    public static int both() {
      return -1; // -1 means both parts
    }
  }

  @Category(AOC)
  @Name(Parse.NAME)
  @Label("Parse")
  @Description("Parsing of AOC input")
  @StackTrace(value = false)
  public static class Parse extends Event {

    static final String NAME = "aoc.Parse";

    public static Parse start(Part part) {
      Parse event = new Parse();
      event.part = part.ordinal() + 1;
      event.begin();
      return event;
    }

    public static Parse start() {
      Parse event = new Parse();
      event.part = Part.both();
      event.begin();
      return event;
    }

    @Label("Part")
    @Description("Which part of AOC puzzle solution this relates to (-1 means it applies to all parts)")
    int part;

    @Label("Parse")
    @Description("Custom parse stage, if any (e.g. 'rules', 'updates', etc.)")
    String stage;
  }

  @Category(AOC)
  @Name(Compute.NAME)
  @Label("Calculate")
  @Description("Captures single span of AOC computation pass")
  @StackTrace(value = false)
  public static class Compute extends Event {

    static final String NAME = "aoc.Compute";

    public static Compute start(Part part) {
      var event = new Compute();
      event.part = part.ordinal() + 1;
      event.begin();
      return event;
    }

    public static Compute start() {
      var event = new Compute();
      event.part = Part.both();
      event.begin();
      return event;
    }

    @Label("Part")
    @Description("Which part of AOC puzzle solution this relates to (-1 means it applies to all parts)")
    int part;

    @Label("Stage")
    @Description("Custom parse stage, if any (e.g. 'rules', 'updates', etc.)")
    String stage;
  }

  @Category(AOC)
  @Name(Collect.NAME)
  @Label("Collect")
  @Description("Captures single span of AOC collection pass")
  public static class Collect extends Event {

    static final String NAME = "aoc.Collect";

    public static Collect start(Part part) {
      var event = new Collect();
      event.part = part.ordinal() + 1;
      event.begin();
      return event;
    }

    public static Collect start() {
      var event = new Collect();
      event.part = Part.both();
      event.begin();
      return event;
    }

    @Label("Part")
    @Description("Which part of AOC puzzle solution this relates to (-1 means it applies to all parts)")
    int part;

    @Label("Stage")
    @Description("Custom parse stage, if any (e.g. 'rules', 'updates', etc.)")
    String stage;
  }

  private final ConcurrentLinkedQueue<RecordedEvent> events = new ConcurrentLinkedQueue<>();
  private final RecordingStream rs;
  private final long traceStart;

  public Trace() {
    this.rs = new RecordingStream();

    this.rs.enable(Parse.NAME).withoutThreshold().withoutStackTrace();
    this.rs.enable(Collect.NAME).withoutThreshold().withoutStackTrace();
    this.rs.enable(Compute.NAME).withoutThreshold().withoutStackTrace();

    this.rs.onEvent(Parse.NAME, events::add);
    this.rs.onEvent(Collect.NAME, events::add);
    this.rs.onEvent(Compute.NAME, events::add);

    this.rs.startAsync();
    this.traceStart = System.nanoTime();
  }

  public Stats stop() {
    long totalNanos = System.nanoTime() - this.traceStart;
    rs.stop();

    var labels = new HashMap<String, String>(2, 1.0f);

    var totalTimes = new long[Part.count()];
    var parseTimes = new long[Part.count()];
    var collectTimes = new long[Part.count()];
    var computeTimes = new long[Part.count()];

    for (RecordedEvent event : this.events) {
      var eventName = event.getEventType().getName();
      labels.computeIfAbsent(eventName, _ -> event.getEventType().getLabel());

      var nanoTime = event.getDuration().toNanos();
      int part = event.getInt("part");

      for (Part p : Part.values()) {
        if (p.isSet(part)) {
          var ordinal = p.ordinal();
          totalTimes[ordinal] += nanoTime;
          switch (eventName) {
            case Parse.NAME -> parseTimes[ordinal] += nanoTime;
            case Collect.NAME -> collectTimes[ordinal] += nanoTime;
            case Compute.NAME -> computeTimes[ordinal] += nanoTime;
          }
        }
      }
    }

    PartStats[] partStats = new PartStats[Part.count()];
    for (Part part : Part.values()) {
      int ordinal = part.ordinal();

      var parseLabel = labels.get(Parse.NAME);
      var parseTime = parseTimes[ordinal];

      var collectLabel = labels.get(Collect.NAME);
      var collectTime = collectTimes[ordinal];

      var computeLabel = labels.get(Compute.NAME);
      var computeTime = computeTimes[ordinal];

      partStats[ordinal] = new PartStats(
          new HumanTime(totalTimes[ordinal]),
          parseTime > 0 ? new NamedStats(parseLabel, new HumanTime(parseTime)) : null,
          collectTime > 0 ? new NamedStats(collectLabel, new HumanTime(collectTime)) : null,
          computeTime > 0 ? new NamedStats(computeLabel, new HumanTime(computeTime)) : null);
    }

    return new Stats(
        new HumanTime(totalNanos),
        partStats[Part1.ordinal()],
        partStats[Part2.ordinal()]);
  }

  public void close() {
    rs.close();
  }


  public record Stats(HumanTime totalTime, PartStats part1, PartStats part2) {

    public String formatResult(Part part, long result) {
      var out = part.toString() + ": " + result;
      if (part == Part.Part1 && part1 != null) {
        out += " (" + part1 + ")";
      } else if (part == Part.Part2 && part2 != null && part2.computeStats != null) {
        out += " (" + part2 + ")";
      }
      return out;
    }
  }

  public record PartStats(
      HumanTime totalTime,
      NamedStats parseStats,
      NamedStats collectStats,
      NamedStats computeStats) {

    @Override
    public String toString() {
      if (parseStats == null && collectStats == null && computeStats == null) {
        return totalTime.toString();
      }

      return totalTime +
          (parseStats != null ? "; " + parseStats : "") +
          (collectStats != null ? "; " + collectStats : "") +
          (computeStats != null ? "; " + computeStats : "");
    }
  }

  public record NamedStats(String name, HumanTime time) {

    @Override
    public String toString() {
      return name + ": " + time;
    }
  }
}
