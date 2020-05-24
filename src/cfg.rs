use clap::clap_app;

use crate::errors::*;

use std::fs::{remove_file, rename, File};
use std::io;
use std::io::prelude::*;
use std::io::{stdout, BufReader};
use std::path::{Path, PathBuf};

pub fn read_args() -> Result<Config, Error> {
    let args = clap_app!(splice =>
        (version: "0.0.1")
        (author: "tillyboy")
        (about: "Splicing of text files")
        (@arg SPLICEID: +required "Sets ID of the intron (part that will be spliced)")
        (@arg FILENAME: +required "Sets the input file to splice")
        (@arg INSERT: -i --insert +takes_value "Inserts it's argument into the Text")
        (@arg MUTATE: -m --mutate "Switches to in-place editing")
        (@arg EXONS: -e --exons "Switches from printing introns to printing exons")
        // (@arg OUTPUT: -o --output "Specifies output file")
    )
    .get_matches();

    // TODO: better notation as soon as try_trait implemented for Option in upstream
    // deconstruct args, construct config
    let codon = match args.value_of("SPLICEID") {
        Some(v) => v,
        None => return Err(Error::new(UNDEF_PARSE_ERROR, None)),
    };
    let cfg = Config {
        filename: PathBuf::from(match args.value_of("FILENAME") {
            Some(v) => v,
            None => return Err(Error::new(UNDEF_PARSE_ERROR, None)),
        }),
        start_codon: format!("spliceSTART {}", codon),
        stop_codon: format!("spliceSTOP {}", codon),
        insert: args.value_of("INSERT").map(|s| String::from(s)),
        mutate: args.is_present("MUTATE"),
        exons: args.is_present("EXONS"),
        outfile: None,
    };

    // check if valid
    if cfg.exons && cfg.insert.is_some() {
        return Err(Error::new(INVALID_ARGS_ERROR, Some(String::from("i + e"))));
        // return Err(format!("{}: {} + {}", INVALID_ARGS_ERROR, 'i', 'e'));
    }
    // good to go
    Ok(cfg)
}

#[derive(Debug)]
pub struct Config {
    filename: PathBuf,
    pub start_codon: String,
    pub stop_codon: String,
    pub insert: Option<String>,
    mutate: bool,
    pub exons: bool,
    outfile: Option<PathBuf>,
}

impl Config {
    // TODO: STDIN support
    pub fn get_reader(&self) -> Result<BufReader<File>, Error> {
        let path = Path::new(&self.filename);
        let file = match File::open(&path) {
            Err(why) => {
                return Err(Error::new(
                    OPEN_FILE_ERROR,
                    Some(format!(
                        "{:?}: {}",
                        self.filename.to_str().unwrap(),
                        why.to_string()
                    )),
                ))
            }
            Ok(file) => file,
        };
        return Ok(BufReader::new(file));
    }

    // for multiple output files, this should be in a tmpdir
    pub fn get_writer(&mut self) -> io::Result<WriterBox> {
        let mybox: WriterBox;
        if self.outfile.is_some() {
            // simply assume that user doesn't try to splice root
            self.outfile = Some(PathBuf::from(format!(
                "{}.splice",
                self.filename.to_str().unwrap()
            )));
            // TODO: remove unwraps
            mybox = Box::new(File::create(self.outfile.as_ref().unwrap()).unwrap());
        } else {
            mybox = Box::new(stdout());
        }
        Ok(mybox)
    }

    pub fn finish(&self) -> io::Result<()> {
        if self.mutate {
            remove_file(&self.filename)?;
            rename(self.outfile.as_ref().unwrap(), &self.filename)?;
        }
        Ok(())
    }
}
type WriterBox = Box<dyn Write>;
