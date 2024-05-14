use std::thread;
use std::time::Duration;
use rand::Rng;
extern crate num_cpus;

fn main() {
   let core_count = num_cpus::get();
   let handle = thread::spawn(|| 
        {
            for i in 1..num_cpus::get(){
                let local_random: i32 = rand::thread_rng().gen_range(1..2147483646);
                println!("number {i} from spawned thread! I choose... {local_random}!");
                thread::sleep(Duration::from_millis(2));
            }
        }
    );


    println!("Hello, world!");

    handle.join().unwrap();
    println!("All threads finished!")
}
