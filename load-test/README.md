# Load Testing

```bash
cargo run --release --bin load-test -- -H http://localhost:3000/ -u 1000 -r 10 --run-time 30s
```

-u: number of users
-r: hatch rate
--run-time: duration
