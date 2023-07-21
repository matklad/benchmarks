import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.time.Instant;
import java.time.Duration;

public class Main {
    private static final long N = 50_000_000;

    public static void main(String[] args) {
        List<String> v = new ArrayList<>();

        Instant t = Instant.now();
        for (long i = 0; i < N; i++) {
            v.add(Long.toString(i));
        }
        System.out.println(Duration.between(t, Instant.now()).toMillis());

        t = Instant.now();
        Collections.sort(v);
        System.out.println(Duration.between(t, Instant.now()).toMillis());

        t = Instant.now();
        long total = 0;
        for (long i = 0; i < N; i++) {
            total += Collections.binarySearch(v, Long.toString(i)) >= 0 ? 1 : 0;
        }
        System.out.println(Duration.between(t, Instant.now()).toMillis());

        System.out.println(total);
    }
}
