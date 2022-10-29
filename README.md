## Rust + Tokio Example

Now, start the server:
```
$ cargo run
```
and in a separate terminal window, run the hello-redis example:
```
$ cargo run --example hello-redis
```
Now, the output will be:
```
result = b"under the dev"
```
We can now get and set values, but there is a problem: The values are not shared between connections. If another socket connects and tries to GET the hello key, it will not find anything.