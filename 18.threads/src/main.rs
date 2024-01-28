use std::thread;

fn main() {
    let handle = thread::spawn(move || {
        // do stuff
    });

    // do stuff simultaneously in the main thread

    // wait until thread has ended (exited)
    handle.join().unwrap(); // Remembering that unwrap will crash the program on error
}
