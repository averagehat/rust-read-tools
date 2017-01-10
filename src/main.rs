
extern crate docopt;
extern crate bio;
extern crate rustc_serialize;

use std::io;
use std::fs::File;
use std::path::Path;
use bio::io::fasta;


use docopt::Docopt;

 // readtools <fasta> [--gene=<gene>] [--maxns=<maxns>]
//struct Args { //   maxNs: i8, //   gene:   String, //   input: Path //}
//docopt!(Args derive Debug, "
const USAGE: &'static str =  "
readtools.

Usage:
  readtools dropns  <fasta> <maxns>
  readtools dropgene <fasta> <gene> 

Options:
  -h --help     Show this screen.
  --version     Show version.
  --maxns=<maxns>  Maximum number of Ns.
  --gene=<gene>      gene to separate out.
  --drifting    Drifting mine.
";
#[derive(Debug, RustcDecodable)]
struct Args {
  cmd_dropns: bool,
  cmd_dropgene: bool,
  arg_fasta: String,
  arg_maxns: usize,
  arg_gene:   String
}

#[derive(Debug)]
enum RunType {
  Ns,
  Gene
}
fn main() {
 run();
}
fn run() -> Result<(), io::Error> {
    //let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit()); 
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    let input = Path::new(&args.arg_fasta);
    let fasta_in = fasta::Reader::new(try!(File::open(&input)));
    let recs = fasta_in.records();
    
    let runtype = if args.cmd_dropns { RunType::Ns } else { RunType::Gene };
    let (no_reads, with_reads): (Vec<_>, Vec<_>) = match &runtype {
      &RunType::Ns =>   recs.partition(|&ref r_| {
          let r = r_.as_ref();
          r.unwrap().seq().iter().filter(|&x| (x == &b'n' || x == &b'N')).count() <= args.arg_maxns
                                       }),
      &RunType::Gene => {
          let pattern = format!("|{}|", &args.arg_gene);
          recs.partition(|&ref r_| {
              let r = r_.as_ref();
              !r.unwrap().id().unwrap().contains(&pattern) }) 
      }
  };
    let prefix =  input.file_stem().unwrap().to_str().unwrap();
    let np = format!("{:}-no-{:?}.fas", prefix, runtype);
    let wp = format!("{:}-with-{:?}.fas", prefix, runtype);
    let no_path = Path::new(&np);
    let with_path = Path::new(&wp);
    let mut no_writer =   fasta::Writer::new(try!(File::create(&no_path)));
    let mut with_writer = fasta::Writer::new(try!(File::create(&with_path)));
    for r in no_reads {
      try!(no_writer.write_record(&r.unwrap()));
}
    for r in with_reads {
      try!(with_writer.write_record(&r.unwrap()));
} 
    try!(no_writer.flush());
    try!(with_writer.flush());
    Ok(())

}

