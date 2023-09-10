Simple example for a producer/consumer using pipes

Illustrates how a pipe is just a byte buffer and the consumer can process async, as bytes are received. In this example the consumer is using newlines as the signal (via `lines()`) to read from stdin.

The consumer output will happen every 2 seconds, showing that it's not waiting for the producer to finish.
```sh
$ cargo build --release --bins
$ ./target/release/producer -w 2s "This is test {{i}}" | ./target/release/consumer "Consumer: "
Consumer: This is test 0
Consumer: This is test 1
Consumer: This is test 2
Consumer: This is test 3
Consumer: This is test 4
```

To see the two output streams interleaved:
```sh
$ ./target/release/producer -w 2s "This is test {{i}}" | tee /dev/fd/2 | ./target/release/consumer "Consumer: "
This is test 0
Consumer: This is test 0
This is test 1
Consumer: This is test 1
This is test 2
Consumer: This is test 2
This is test 3
Consumer: This is test 3
This is test 4
Consumer: This is test 4
```

