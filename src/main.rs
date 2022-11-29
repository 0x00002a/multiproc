use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let to_run = args
        .get(1)
        .expect("need number to run")
        .parse()
        .expect("qty to run must be number");
    let cmd = args.get(2).expect("need command to run");
    let cmd_args: Vec<_> = args.iter().skip(3).collect();
    std::thread::scope(|s| {
        #[allow(unused_variables)]
        let processes: Vec<_> = (0..to_run)
            .map(|n| {
                let cmd_args = &cmd_args;
                s.spawn(move || {
                    let child = process::Command::new(cmd)
                        .args(cmd_args)
                        .stdout(process::Stdio::piped())
                        .spawn()
                        .expect(&format!("failed to spawn child {:03}", n + 1));
                    let stdout = child.stdout.unwrap();
                    BufReader::new(stdout).lines().for_each(|l| {
                        println!("[child {:03}]: {}", n + 1, l.unwrap());
                    });
                });
            })
            .collect();
    });
}
