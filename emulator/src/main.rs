use std::{env, fs, io};

use wasmi::{Engine, Module};
use wasmi_wasi::sync::{self, WasiCtxBuilder};

type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
enum Error {
    Wasmi(wasmi::Error),
    Io(io::Error),
    MissingArg,
}

impl From<wasmi::Error> for Error {
    fn from(error: wasmi::Error) -> Self {
        Self::Wasmi(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

struct HostState;

fn main() -> Result {
    // Load the wasm file (WASI)
    let path = env::args().skip(1).next().ok_or(Error::MissingArg)?;
    let wasm = fs::read(path)?;

    // Create engine and load module
    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..])?;

    sync::add_to_linker(&mut linker, WasiCtxBuilder::new().build());

    /*
    let mut store = Store::new(&engine, HostState);
    let host_hello = Func::wrap(&mut store, |caller: Caller<'_, HostState>, param: i32| {
        println!("Got {param} from WebAssembly");
        println!("My host state is: {}", caller.data());
    });

    // In order to create Wasm module instances and link their imports
    // and exports we require a `Linker`.
    let mut linker = <Linker<HostState>>::new(&engine);
    // Instantiation of a Wasm module requires defining its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    //
    // Also before using an instance created this way we need to start it.
    linker.define("host", "hello", host_hello)?;
    let instance = linker
        .instantiate(&mut store, &module)?
        .start(&mut store)?;
    let hello = instance.get_typed_func::<(), ()>(&store, "hello")?;

    // And finally we can call the wasm!
    hello.call(&mut store, ())?;*/

    Ok(())
}
