fn main() {
  let f = future::poll_fn(
      move || match tokio_threadpool::blocking(|| f.poll()).unwrap() {
          Async::Ready(v) => v,
          Async::NotReady => Ok(Async::NotReady),
      },
  );
}