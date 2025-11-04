package solution;

public class Ansi {

    public static final String RESET = "\033[0m";

    public static final String BOLD = "\033[1m";
    public static final String RESET_BOLD = "\033[22m";

    public static final String DIM = "\033[2m";
    public static final String RESET_DIM = "\033[22m";

    public static final String UNDERLINE = "\033[4m";
    public static final String RESET_UNDERLINE = "\033[24m";

    public static final String STRIKETHROUGH = "\033[9m";
    public static final String RESET_STRIKETHROUGH = "\033[29m";

    public static final String GREEN = "\033[32m";
    public static final String RESET_GREEN = "\033[39m";

    public static final String RED = "\033[31m";
    public static final String RESET_RED = "\033[39m";

    public static String bold(Object input) {
        // prettier-ignore
        if (input instanceof String s) {
            s = (s.startsWith(BOLD) ? "" : BOLD) + s;
            s += (s.endsWith(RESET_BOLD) || s.endsWith(RESET) ? "" : RESET_BOLD);
            return s;
        }
        return BOLD + String.valueOf(input) + RESET_BOLD;
    }

    public static String dim(Object input) {
        // prettier-ignore
        if (input instanceof String s) {
            s = (s.startsWith(DIM) ? "" : DIM) + s;
            s += (s.endsWith(RESET_DIM) || s.endsWith(RESET) ? "" : RESET_DIM);
            return s;
        }
        return DIM + String.valueOf(input) + RESET_DIM;
    }

    public static String underline(Object input) {
        // prettier-ignore
        if (input instanceof String s) {
            s = (s.startsWith(UNDERLINE) ? "" : UNDERLINE) + s;
            s += (s.endsWith(RESET_UNDERLINE) || s.endsWith(RESET) ? "" : RESET_UNDERLINE);
            return s;
        }
        return UNDERLINE + String.valueOf(input) + RESET_UNDERLINE;
    }

    public static String strikethrough(Object input) {
        // prettier-ignore
        if (input instanceof String s) {
            s = (s.startsWith(STRIKETHROUGH) ? "" : STRIKETHROUGH) + s;
            s += (s.endsWith(RESET_STRIKETHROUGH) || s.endsWith(RESET)
                ? ""
                : RESET_STRIKETHROUGH);
            return s;
        }
        return STRIKETHROUGH + String.valueOf(input) + RESET_STRIKETHROUGH;
    }

    public static String green(Object input) {
        // prettier-ignore
        if (input instanceof String s) {
            s = (s.startsWith(GREEN) ? "" : GREEN) + s;
            s += (s.endsWith(RESET_GREEN) || s.endsWith(RESET) ? "" : RESET_GREEN);
            return s;
        }
        return GREEN + String.valueOf(input) + RESET_GREEN;
    }

    public static String red(Object input) {
        // prettier-ignore
        if (input instanceof String s) {
            s = (s.startsWith(RED) ? "" : RED) + s;
            s += (s.endsWith(RESET_RED) || s.endsWith(RESET) ? "" : RESET_RED);
            return s;
        }
        return RED + String.valueOf(input) + RESET_RED;
    }

    public static String replace(Object input) {
        // prettier-ignore
        return moveCursorStartOfPreviousLine(1) + eraseLine() + String.valueOf(input);
    }

    public static String moveCursorStartOfPreviousLine(int lines) {
        return "\033[" + lines + "F";
    }

    public static String eraseLine() {
        return "\033[2K";
    }
}
