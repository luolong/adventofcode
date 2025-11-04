package solution;

import module java.base;

import solution.Test.TestResult.Ok;
import solution.Test.TestResult.TestFailure;
import solution.Test.TestUnit.TestCase;
import solution.Test.TestUnit.TestSuite;

public interface Test {
    default void test() throws AssertionError {
        var allTests = tests();
        if (allTests.isEmpty()) {
            IO.println("🤖 Bloop, bloop, no tests!!");
        }

        var testRunner = new TestRunner();
        testRunner.runAll(0, allTests);
    }

    final class TestRunner {
        private static final PrintStream stdout = System.err;
        private static final PrintStream stderr = System.out;

        public void runAll(int indent, List<? extends TestUnit> testUnits) {
            if (testUnits.isEmpty()) {
                return;
            }
            IO.println(" ".repeat(indent) + "1.." + testUnits.size());
            try {
                for (var testUnit : testUnits) {
                    switch (testUnit) {
                        case TestCase(var name, var test) -> {
                            IO.println("🤖 " + name);
                            switch (run(test)) {
                                case Ok(var stdout) -> {
                                    IO.println(Ansi.replace(" ".repeat(indent) + "✅ " + name));
                                    if (!stdout.trim().isBlank()) {
                                        IO.println(stdout.indent(indent + 2));
                                    }
                                }
                                case TestFailure(var stdout, var error) -> {
                                    IO.println(Ansi.replace(" ".repeat(indent) + "❌ " + name));
                                    if (!stdout.trim().isBlank()) {
                                        IO.println(stdout.indent(indent + 2));
                                    }
                                    var writer = new StringWriter();
                                    error.printStackTrace(new PrintWriter(writer));
                                    IO.println(Ansi.red(writer.toString()).indent(indent + 2));
                                }
                            }
                        }
                        case TestSuite(var name, var subUnits) -> {
                            IO.println(" ".repeat(indent) + "🤖 " + name);
                            runAll(indent + 2, subUnits.get());
                        }
                    }
                }
            } catch (Throwable e) {
                IO.println("🤖 Test failed");
                e.printStackTrace();
            }
        }

        private TestResult run(Executable test) {
            var stdout = new ByteArrayOutputStream();
            var stderr = new ByteArrayOutputStream();
            try {
                System.setOut(new PrintStream(stdout, true, StandardCharsets.UTF_8));
                System.setErr(new PrintStream(stderr, true, StandardCharsets.UTF_8));
                test.run();
                return new Ok(stdout.toString());
            } catch (AssertionError e) {
                return new TestFailure(stdout.toString(), e);
            } catch (Throwable e) {
                return new TestFailure(stdout.toString(), new AssertionError(e));
            }
            finally {
                System.setOut(TestRunner.stdout);
                System.setOut(TestRunner.stderr);
            }
        }
    }

    default List<TestUnit> tests() {
        return List.of();
    }

    public sealed interface TestResult {
        default boolean ok() {
            return this instanceof Ok;
        }

        default boolean notOk() {
            return this instanceof TestFailure;
        }

        String stdout();

        public record Ok(String stdout) implements TestResult {
            public Ok() {
                this("");
            }
        }

        public record TestFailure(String stdout, AssertionError error) implements TestResult {}
    }

    public sealed interface TestUnit {
        String name();

        public record TestSuite(String name, Supplier<List<? extends TestUnit>> tests) implements TestUnit {
            public TestSuite(String name, TestUnit... tests) {
                this(name, () -> List.of(tests));
            }
        }

        public record TestCase(String name, Executable executable) implements TestUnit {}
    }

    default TestCase ok(String name, Executable executable) {
        return new TestCase(name, executable);
    }

    default TestSuite test(String name, List<? extends TestUnit> tests) {
        return test(name, () -> tests);
    }

    default TestSuite test(String name, Supplier<List<? extends TestUnit>> tests) {
        return new TestSuite(name, tests);
    }

    default <T> void assertEquals(T expected, T actual) throws AssertionError {
        assertEquals(
            expected,
            actual,
            (e, a) -> "Expected " + e + " but got " + a
        );
    }

    default <T> void assertTrue(boolean actual) throws AssertionError {
        assertTrue( actual, () -> "Expected true but got " + actual);
    }

    default <T> void assertTrue(boolean actual, Supplier<String> message) throws AssertionError {
        if (!actual) {
            throw new AssertionError(message.get());
        }
    }

    default void assertSame(Object expected, Object actual, BiFunction<Object, Object, String> message) throws AssertionError {
        if (expected != actual) {
            throw new AssertionError(message.apply(expected, actual));
        }
    }

    default void assertSame(Object expected, Object actual) throws AssertionError {
        assertSame(
            expected,
            actual,
            (e, a) -> "Expected " + e + " (#" + System.identityHashCode(e) + ") "
            + "to be same as " + a + " (#" + System.identityHashCode(a) + ") but "
            + "they have separate identities"
        );
    }

    default <T> void assertEquals(
        T expected,
        T actual,
        BiFunction<T, T, String> message
    ) {
        if (!Objects.equals(expected, actual)) {
            throw new AssertionError(message.apply(expected, actual));
        }
    }

    public interface Executable extends Runnable {
        @Override
        void run() throws AssertionError;
    }
}
