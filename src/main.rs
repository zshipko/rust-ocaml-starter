// Read the corresponding .ml file first.

ocaml::import! {
    fn hello_world() -> String;
}

#[derive(ocaml::ToValue, ocaml::FromValue, Debug)]
pub enum T {
    A,
    B(ocaml::Int),
}

ocaml::import! {
    fn maybe_inc(t: T) -> T;
}

// If the OCaml type `MyStruct.t` included a value we couldn't (or didn't want
// to) wrap, we would use `ocaml::Value` for it.
#[derive(ocaml::ToValue, ocaml::FromValue, Debug)]
pub struct MyStructT {
    a: ocaml::Float,
    b: ocaml::Int,
}

ocaml::import! {
    // We can optionally make input args references, allowing us to reuse a
    // value in Rust.
    // This does not require any changes to the OCaml code.
    fn mystruct_inc_both(t: &MyStructT) -> MyStructT;
}

// Run the non-async examples.
fn run(rt: &ocaml::Runtime) {
    unsafe {
        println!("hello_world: {}", hello_world(&rt).unwrap());
        println!("maybe_inc: {:?}", maybe_inc(&rt, T::B(1)).unwrap());
        println!(
            "mystruct_inc_both: {:?}",
            mystruct_inc_both(&rt, &MyStructT { a: 1.0, b: 2 }).unwrap()
        );
    }
}

// This struct lets us wrap an OCaml Lwt.t value in a Rust future.
pub struct Lwt<'a, T> {
    rt: &'a ocaml::Runtime,
    value: ocaml::Value,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T> Lwt<'a, T> {
    pub fn new(rt: &'a ocaml::Runtime, value: ocaml::Value) -> Self {
        Self {
            rt,
            value,
            _phantom: std::marker::PhantomData,
        }
    }

    ocaml::import! {
      fn lwt_poll(x: &ocaml::Value) -> Option<ocaml::Value>;
    }
}

impl<'a, T: ocaml::FromValue + Unpin> std::future::Future for Lwt<'a, T> {
    type Output = Result<T, ocaml::Error>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context,
    ) -> std::task::Poll<Self::Output> {
        match unsafe { Self::lwt_poll(&self.rt, &self.value) } {
            Ok(Some(x)) => {
                return std::task::Poll::Ready(Ok(ocaml::FromValue::from_value(x)));
            }
            Ok(None) => {
                // We immediately mark this future as ready to be polled again.
                // This will lead to busy-polling, which could cause a CPU core to be
                // pegged to 100%.
                // The proper solution is probably to thread through the Lwt waiter
                // signal, but I'll leave that to someone else.
                ctx.waker().wake_by_ref();
                return std::task::Poll::Pending;
            }
            Err(e) => {
                return std::task::Poll::Ready(Err(e));
            }
        }
    }
}

ocaml::import! {
  // TODO: Can we make this return a `Lwt<'a, String>`, or at least something
  // which better conveys the fact that it's an Lwt.t?
  fn fetch_example_dot_com() -> ocaml::Value;
}

// Run the async example.
async fn run_lwt(rt: &ocaml::Runtime) {
    let future: Lwt<String> = Lwt::new(&rt, unsafe { fetch_example_dot_com(&rt) }.unwrap());
    println!("Future created but not yet polled. Polling now.");
    let value = future.await.unwrap();
    println!("Future resolved:\n{}", value);
}

#[tokio::main]
async fn main() {
    let rt = ocaml::runtime::init();
    println!("# Running non-async examples.");
    run(&rt);
    println!("\n# Running async example.");
    run_lwt(&rt).await;
}
