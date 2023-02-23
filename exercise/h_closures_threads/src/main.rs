use crossbeam::channel::{self, unbounded};
use std::thread;
use std::time::Duration;

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");

    v.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    let handle = thread::spawn(|| expensive_sum(my_vector));

    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", letter);
        pause_ms(200);
    }

    let sum = handle.join().unwrap();

    println!("The child thread's expensive sum is {}", sum);

    //use channels
    let  (s, r)  = crossbeam::channel::unbounded();

    let s1 = s.clone();

    let handle_a = thread::spawn(move || {
        let vals = vec!["one","two","three","four","five",];
        for v in vals {
            s1.send(v).unwrap();
            pause_ms(1000);
        }
    });

    pause_ms(1000);

   let handle_b = thread::spawn(move || {
        let vals = vec!["more", "messages", "to", "you"];
        for v in vals {
            s.send(v).unwrap();
            pause_ms(1000); 
        }
    });

    for mess in r {
        println!("Got: {}", mess);
    }

    handle_a.join().unwrap(); // not necessari
    handle_b.join().unwrap();


    //create one sender in main and two receivers in child threads
    let  (s2, r2)  = crossbeam::channel::unbounded();
    let r3 = r2.clone();


    let rec_a  = thread::spawn( move || {
        for message in r2 {
            println!("receiver A got: {}", message);
        }
    });
    let rec_b  = thread::spawn( move || {
        for message in r3 {
            println!("receiver B got: {}", message);
        }
    });

    
    let my_vector2 = vec![200, 500, 100, 400, 455, 378];

    for v in my_vector2 {
        s2.send(v).unwrap();
        pause_ms(50);
    }

    drop(s2); //need to close channel so receiving ends will break their loops or else it will wait forever

    rec_a.join().unwrap();
    rec_b.join().unwrap();
    println!("Main thread: Exiting.");


}
