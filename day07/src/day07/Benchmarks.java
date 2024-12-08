package day07;

import org.openjdk.jmh.annotations.*;
import org.openjdk.jmh.results.format.ResultFormatType;
import org.openjdk.jmh.runner.options.OptionsBuilder;

import java.math.BigInteger;
import java.util.concurrent.TimeUnit;

@BenchmarkMode(Mode.AverageTime)
@OutputTimeUnit(TimeUnit.NANOSECONDS)
@Warmup(iterations = 5, time = 5, timeUnit = TimeUnit.MILLISECONDS)
@Measurement(iterations = 5, time = 5, timeUnit = TimeUnit.MILLISECONDS)
@State(Scope.Benchmark)
public class Benchmarks {
    private static long[] POWER_OF_10 = {
            1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
            10000000000L, 100000000000L, 1000000000000L, 10000000000000L, 100000000000000L,
            1000000000000000L, 10000000000000000L, 100000000000000000L, 1000000000000000000L
    };

    @Param({ "0", "22", "999999999" })
    public long a;

    @Param({ "0", "4444", "7777777" })
    public long b;

    @Benchmark
    public long stringConcatenation() {
        return Long.parseLong(String.valueOf(a) + b);
    }

    @Benchmark
    public long log10() {
        if (a == 0) {
            return b;
        }
        if (b == 0) {
            return a * 10;
        }

        var magnitude = (Math.log10(b) + 1);
        var multiplier = POWER_OF_10[(int) magnitude];
        return Math.addExact(a * multiplier, b);
    }

    public static void main(String[] args) {
        var A = new long[] { 0, 22, 999999999 };
        var B = new long[] { 0, 4444, 7777777 };

        var bench = new Benchmarks();
        for (var a : A) {
            for (var b : B) {
                System.out.printf("Given a = %d, b = %d: ", a, b);
                bench.a = a;
                bench.b = b;
                System.out.printf("String concatenation = %d; ", bench.stringConcatenation());
                System.out.printf("Log10 concatenation = %d", bench.log10());

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
