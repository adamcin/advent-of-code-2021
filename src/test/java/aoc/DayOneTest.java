package aoc;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import com.google.common.collect.Streams;

import org.junit.jupiter.api.Test;

public class DayOneTest {

    @Test
    public void getAnswer() throws Exception {
        Path input = Paths.get("data", "day-01", "input.txt");

        final List<Integer> values = Files.readAllLines(input).stream()
                .map(Integer::parseInt).collect(Collectors.toList());

        final List<Optional<Integer>> previous = Stream.concat(
                Stream.of(Optional.<Integer>empty()),
                values.subList(0, values.size() - 1).stream().map(Optional::<Integer>of))
                .collect(Collectors.toList());

        final List<Integer> increases = Streams.zip(values.stream(), previous.stream(),
                (curr, optPrev) -> optPrev.map(prev -> curr - prev))
                .filter(Optional::isPresent)
                .map(Optional::get)
                .filter(value -> value > 0)
                .collect(Collectors.toList());

        assertEquals(1557, increases.size(), "expect increases size");
    }

    @Test
    public void getAnswer2() throws Exception {
        Path input = Paths.get("data", "day-01", "input.txt");

        final List<Integer> values = Files.readAllLines(input).stream()
                .map(Integer::parseInt).collect(Collectors.toList());

        final List<Optional<Integer>> previous = Stream.concat(
                Stream.of(Optional.<Integer>empty()),
                values.subList(0, values.size() - 1).stream().map(Optional::<Integer>of))
                .collect(Collectors.toList());

        final List<Optional<Integer>> penprev = Stream.concat(
                Stream.concat(Stream.of(Optional.<Integer>empty()), Stream.of(Optional.<Integer>empty())),
                values.subList(0, values.size() - 2).stream().map(Optional::<Integer>of))
                .collect(Collectors.toList());

        final List<Optional<Integer>> leftSums = Streams.zip(values.stream(), previous.stream(),
                (curr, optPrev) -> optPrev.map(prev -> curr + prev))
                .collect(Collectors.toList());

        final List<Integer> sums = Streams.zip(leftSums.stream(), penprev.stream(),
                (optLeft, optPrev) -> optLeft.flatMap(left -> optPrev.map(prev -> left + prev)))
                .filter(Optional::isPresent)
                .map(Optional::get)
                .collect(Collectors.toList());

        final List<Optional<Integer>> prevSums = Stream.concat(
            Stream.of(Optional.<Integer>empty()),
            sums.subList(0, sums.size() - 1).stream().map(Optional::<Integer>of))
            .collect(Collectors.toList());

        final List<Integer> increases = Streams.zip(sums.stream(), prevSums.stream(),
                (curr, optPrev) -> optPrev.map(prev -> curr - prev))
                .filter(Optional::isPresent)
                .map(Optional::get)
                .filter(value -> value > 0)
                .collect(Collectors.toList());

        assertEquals(1608, increases.size(), "expect increases size");
    }
}