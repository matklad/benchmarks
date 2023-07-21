import java.util.HashMap;
import java.time.Instant;
import java.time.Duration;

public class Main {
    public static final long N = 50_000_000;

    public static void main(String[] args) {
        HashMap<String, Long> m = new HashMap<>();
        Instant t = Instant.now();

        for (long i = 0; i < N; i++) {
            m.put(Long.toString(i), i);
        }

        System.out.println(Duration.between(t, Instant.now()).toMillis());

        t = Instant.now();
        long total = 0;

        for (long i = 0; i < N; i++) {
            total += m.get(Long.toString(i));
        }

        System.out.println(Duration.between(t, Instant.now()).toMillis());
        System.out.println(total);
    }
}
