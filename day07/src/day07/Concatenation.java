package day07;

import org.openjdk.jmh.annotations.*;

import java.util.concurrent.TimeUnit;

@BenchmarkMode(Mode.AverageTime)
@OutputTimeUnit(TimeUnit.NANOSECONDS)
@Warmup(iterations = 5, time = 5, timeUnit = TimeUnit.MILLISECONDS)
@Measurement(iterations = 5, time = 5, timeUnit = TimeUnit.MILLISECONDS)
@State(Scope.Benchmark)
public class Concatenation {
    private static final long[] POWER_OF_10 = {
            1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
            10000000000L, 100000000000L, 1000000000000L, 10000000000000L, 100000000000000L,
            1000000000000000L, 10000000000000000L, 100000000000000000L, 1000000000000000000L
    };

    public static final class StringConcatenation {
        public static long concat(long a, long b) {
            return Long.parseLong(String.valueOf(a) + b);
        }
    }

    public static final class Log10Concatenation {
        public static long concat(long a, long b) {
            if (a == 0) {
                return b;
            }

            if (b == 0) {
                return a * 10;
            }

            var magnitude = Math.log10(b) + 1;
            var multiplier = POWER_OF_10[(int) magnitude];
            return Math.addExact(a * multiplier, b);
        }
    }

    public static final class SwitchConcatenation {
        public static long concat(long a, long b) {
            return (a == 0) ? b : switch (b) {
                case 0L -> a * 10;
                case long l when l < 10L -> Math.addExact(a * 10, l);
                case long l when l < 100L -> Math.addExact(a * 100, l);
                case long l when l < 1000L -> Math.addExact(a * 1000, l);
                case long l when l < 10000L -> Math.addExact(a * 10000, l);
                case long l when l < 100000L -> Math.addExact(a * 100000, l);
                case long l when l < 1000000L -> Math.addExact(a * 1000000, l);
                case long l when l < 10000000L -> Math.addExact(a * 10000000, l);
                case long l when l < 100000000L -> Math.addExact(a * 100000000, l);
                case long l when l < 1000000000L -> Math.addExact(a * 1000000000, l);
                case long l when l < 10000000000L -> Math.addExact(a * 10000000000L, l);
                case long l when l < 100000000000L -> Math.addExact(a * 100000000000L, l);
                case long l when l < 1000000000000L -> Math.addExact(a * 1000000000000L, l);
                case long l when l < 10000000000000L -> Math.addExact(a * 10000000000000L, l);
                case long l when l < 100000000000000L -> Math.addExact(a * 100000000000000L, l);
                case long l when l < 1000000000000000L -> Math.addExact(a * 1000000000000000L, l);
                case long l when l < 10000000000000000L -> Math.addExact(a * 10000000000000000L, l);
                case long l when l < 100000000000000000L -> Math.addExact(a * 100000000000000000L, l);
                case long l when l < 1000000000000000000L -> Math.addExact(a * 1000000000000000000L, l);
                default -> throw new IllegalStateException();
            };
        }
    }

    @Param({ "0", "22", "999999999" })
    public long a;

    @Param({ "0", "4444", "7777777" })
    public long b;

    @Benchmark
    public long stringConcatenation() {
        return StringConcatenation.concat(a, b);
    }

    @Benchmark
    public long log10() {
        return Log10Concatenation.concat(a, b);
    }

    @Benchmark
    public long switchConcat() {
        return SwitchConcatenation.concat(a, b);
    }

    public static void main(String[] args) {
        var A = new long[] { 0, 22, 999999999 };
        var B = new long[] { 0, 4444, 7777777 };

        var bench = new Concatenation();
        for (var a : A) {
            for (var b : B) {
                System.out.printf("Given a = %d, b = %d: ", a, b);
                bench.a = a;
                bench.b = b;
                System.out.printf("String concatenation = %d; ", bench.stringConcatenation());
                System.out.printf("Log10 concatenation = %d", bench.switchConcat());

                if (b > 0) {
                    var magnitude = Math.log10(b) + 1;
                    System.out.printf(" (%d * 10^%d + %d ", a, (long) magnitude, b);

                    var multiplier = POWER_OF_10[(int) magnitude];
                    System.out.printf("=> %d * %d + %d ", a, multiplier, b);
                    System.out.printf("=> %d + %d)%n", a * multiplier, b);
                } else {
                    System.out.println();
                }
            }
        }
/*
        new OptionsBuilder()
                .include(Benchmarks.class.getSimpleName())
                .resultFormat(ResultFormatType.JSON)
                .result("day07_benchmarks.json")
                .build();
*/
    }
}
