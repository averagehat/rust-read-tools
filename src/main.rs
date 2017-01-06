extern crate rayon;
extern crate bio;

use std::sync::{Arc, Mutex};
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;
use rayon::prelude::*;
use rayon::par_iter::zip::ZipIter;
use bio::io::fastq;

use bio::io::fastq::Record;

fn is_odd(x: &String) -> bool {
  match x.parse::<i32>() {
    Ok(x) => (x % 2 != 0),
    Err(_) => { println!("bleh!"); false }
    }
  }

fn has_n(read: &Record) -> bool {
    read.seq().contains(&b'N')
}
fn main() {
   run();
   println!("done!");

}

fn run() -> Result<(), io::Error> {
    let r1 = "t1.fastq"; 
    let r2 = "t2.fastq";
    let in1 = fastq::Reader::new(try!(File::open(r1)));
    let in2 = fastq::Reader::new(try!(File::open(r2)));

    let out1 = Path::new("out1.fastq");
    let out2 = Path::new("out2.fastq");
    let mut out_fwd = Arc::new(Mutex::new(fastq::Writer::new(try!(File::create(&out1)))));
    let mut out_rev = Arc::new(Mutex::new(fastq::Writer::new(try!(File::create(&out2)))));

    let chunk_size = 1;
    let mut recs1_ = in1.records();
    let mut recs2_ = in2.records();

    loop {
        let recs1: Vec<_> = recs1_.by_ref().take(chunk_size).collect();
        let recs2: Vec<_> = recs2_.by_ref().take(chunk_size).collect();

        let size_left = recs1.len();

        recs1.into_par_iter().zip(recs2)
          .filter(|pair| {
                let fwd = pair.0.as_ref();
                let rev = pair.1.as_ref();
              !(has_n(fwd.unwrap()) || has_n(rev.unwrap()))
          })
          .for_each(|pair| {
              out_fwd.lock().unwrap().write_record(&pair.0.unwrap());
              out_rev.lock().unwrap().write_record(&pair.1.unwrap());
          });

        if (size_left < chunk_size) { break; }
    }

    try!(out_fwd.lock().unwrap().flush());
    try!(out_rev.lock().unwrap().flush());
    Ok(())


}

