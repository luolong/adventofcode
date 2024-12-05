package aoc;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.AbstractList;
import java.util.Iterator;
import java.util.List;
import java.util.NoSuchElementException;
import java.util.Objects;

/**
 * Flyweight String implementation in java.base/
 *
 * This class implements an alternative to the String class, providing a lightweight
 * representation of strings that can be shared between multiple instances.
 * It is designed to be memory-efficient and suitable for use in scenarios where
 * large amounts of string data need to be processed.
 */
public class Flyweight implements CharSequence {

    public record SplitOnce(Flyweight left, Flyweight right) {
        public static SplitOnce of(Flyweight string, char delimiter) {
            Objects.requireNonNull(string);
            int index = string.indexOf(delimiter);
            if (index == -1) {
                return null;
            }

            return new SplitOnce(
                string.subSequence(0, index),
                string.subSequence(index + 1, string.length())
            );
        }
    }

    public static Flyweight slurp(Path path) throws IOException {
        return new Flyweight(Files.readString(path));
    }

    public static Iterator<Flyweight> lines(Path path) throws IOException {
        Flyweight string = Flyweight.slurp(path);
        return string.lines();
    }

    private final CharSequence value;

    private Flyweight(CharSequence value) {
        Objects.requireNonNull(value);
        this.value = value;
    }

    @Override
    public int length() {
        return value.length();
    }

    @Override
    public char charAt(int index) {
        return value.charAt(index);
    }

    public Iterator<Flyweight> lines() {
        return split('\n');
    }

    public List<Flyweight> splitToList(char delimiter) {
        var separators = new int[length()];
        var count = 0;
        int index = indexOf(delimiter);
        while (index != -1) {
            separators[count++] = index;
            int start = index + 1;
            index = indexOf(delimiter, start);
        }

        final int size = count + 1;
        return new AbstractList<Flyweight>() {
            @Override
            public int size() {
                return size;
            }

            @Override
            public Flyweight get(int index) {
                Objects.checkIndex(index, size);
                int startIndex = index == 0 ? 0 : separators[index - 1] + 1;
                int endIndex = index == size - 1 ? length() : separators[index];

                return new Flyweight(value.subSequence(startIndex, endIndex));
            }
        };
    }

    public Iterator<Flyweight> split(char delimiter) {
        return new Iterator<Flyweight>() {
            private int lineStart = 0;
            private int nextLineFeed = indexOf(delimiter, lineStart);

            @Override
            public boolean hasNext() {
                if (lineStart < length()) {
                    int index = indexOf(delimiter, lineStart);
                    nextLineFeed = (index == -1) ? length() : index;
                }

                return nextLineFeed >= lineStart;
            }

            @Override
            public Flyweight next() {
                if (lineStart <= nextLineFeed) {
                    Objects.checkFromToIndex(lineStart, nextLineFeed, length());

                    var subSequence = subSequence(lineStart, nextLineFeed);

                    lineStart = nextLineFeed + 1;
                    return new Flyweight(subSequence);
                }

                throw new NoSuchElementException();
            }
        };
    }

    public int indexOf(char c, int fromIndex) {
        for (int i = fromIndex; i < value.length(); i++) {
            if (value.charAt(i) == c) {
                return i;
            }
        }
        return -1;
    }

    @Override
    public Flyweight subSequence(int start, int end) {
        return new Flyweight(this.value.subSequence(start, end));
    }

    public boolean isBlank() {
        if (value.length() > 0) {
            for (int i = 0; i < value.length(); i++) {
                if (!Character.isWhitespace(value.charAt(i))) {
                    return false;
                }
            }
        }
        return true;
    }

    public int indexOf(char c) {
        for (int i = 0; i < value.length(); i++) {
            if (value.charAt(i) == c) {
                return i;
            }
        }
        return -1;
    }

    @Override
    public String toString() {
        return value.toString();
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) {
            return true;
        }

        if (obj instanceof Flyweight other) {
            if (value.length() != other.value.length()) {
                return false;
            }
            for (int i = 0; i < value.length(); i++) {
                if (value.charAt(i) != other.value.charAt(i)) {
                    return false;
                }
            }
            return true;
        }

        return false;
    }

    @Override
    public int hashCode() {
        int result = 1;
        for (int i = 0; i < value.length(); i++) {
            result = 31 * result + value.charAt(i);
        }

        return result;
    }

    public SplitOnce splitOnce(char delimiter) {
        return SplitOnce.of(this, delimiter);
    }

    public int parseInt() {
        return Integer.parseInt(value, 0, value.length(), 10);
    }
}
