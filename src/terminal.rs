use std::{io::{Write, stdout}, borrow::BorrowMut};
use console::Term;

struct Counter {
    start: usize,
    end: usize,
    index: usize,
}

impl Counter {

    fn new(start: usize, end: usize) -> Counter {
        Counter {
            start: start,
            end: end,
            index: start.clone(),
        }
    }

    fn incr(&mut self) -> usize {
        return if(self.index + 1 <= self.end) { self.index += 1; self.index } else { panic!("Counter reach end limit!") };
    }

    fn decr(&mut self) -> usize {
        return if(self.index - 1 >= self.start) { self.index -= 1; self.index } else { panic!("Counter reach start limit!") };
    }
}

pub struct Terminal {
    stdout: Term,
    counter: Option<Counter>,
    name: Option<String>
}

impl Terminal {

    pub fn new() -> Terminal {
        Terminal {
            stdout: Term::stdout(),
            counter: None,
            name: None
        }
    }

    pub fn update(&self, text: &str) {
        self.stdout.clear_line();
        print!("\r{text}");
        stdout().flush();
    }

    pub fn print(&self, text: &str) {
        print!("{text}");
        stdout().flush();
    }

    pub fn println(&self, text: &str) {
        println!("{text}");
    }

    pub fn print_below(&self, text: &str) {
        print!("\n{text}");
        stdout().flush();
    }

    pub fn println_below(&self, text: &str) {
        println!("\n{text}");
    }
}

pub trait ImportTerm {

    fn new(import_list_size: usize, target_directory: &str) -> Self;

    fn import_start(&self);
    
    fn update_import(&mut self, next_media: &str);

    fn searching(&mut self);
    
    fn import_done(&self);

}

impl ImportTerm for Terminal {

    fn new(import_list_size: usize, target_directory: &str) -> Self {
        Terminal { 
            stdout: Term::stdout(),
            counter: Some(Counter::new(0, import_list_size)),
            name: Some(target_directory.to_string()),
        }
    }

    fn import_start(&self) {
        match &self.counter {
            Some(counter) => {
                let list_size = counter.end;
                self.print(&format!("Processing {list_size} medias...\nSearching... : 0/{list_size}"));
            },
            None => panic!("No counter avaiilable!")
        }
    }
    
    fn update_import(&mut self, media: &str) {
        match &mut self.counter {
            Some(counter) => {
                let index = counter.incr();
                let list_size = counter.end;
                self.update(&format!("Import {media} : {index}/{list_size}"));
            },
            None => panic!("No counter avaiilable!")
        }
    }

    fn searching(&mut self) {
        match &self.counter {
            Some(counter) => {
                let index = counter.index;
                let list_size = counter.end;
                self.update(&format!("Searching... : {index}/{list_size}"));
            },
            None => panic!("No counter avaiilable!")
        }
    }
    
    fn import_done(&self) {
        match &self.name {
            Some(import_directory) => {
                self.println(&format!("\nImport to {import_directory} ended"));
            },
            None => panic!("No import directory name available!")
        }
    }

}

pub trait ExportTerm {

    fn new(export_list_size: usize, export_directory: &str) -> Self;

    fn export_start(&self);
    
    fn update_export(&mut self, next_media: &str);
    
    fn export_done(&self);

}

impl ExportTerm for Terminal {

    fn new(export_list_size: usize, export_directory: &str) -> Self {
        Terminal { 
            stdout: Term::stdout(),
            counter: Some(Counter::new(0, export_list_size)),
            name: Some(export_directory.to_string()),
        }
    }

    fn export_start(&self) {
        match &self.counter {
            Some(counter) => {
                let list_size = counter.end;
                self.print(&format!("Read {list_size} media paths...\nExport-- : 0/{list_size}"));
            },
            None => panic!("No counter avaiilable!")
        }
    }
    
    fn update_export(&mut self, media: &str) {
        match &mut self.counter {
            Some(counter) => {
                let index = counter.incr();
                let list_size = counter.end;
                self.update(&format!("Export {media} : {index}/{list_size}"));
            },
            None => panic!("No counter avaiilable!")
        }
    }
    
    fn export_done(&self) {
        match &self.name {
            Some(export_directory) => {
                self.println(&format!("\nExport to {export_directory} ended"));
            },
            None => panic!("No export directory name available!")
        }
    }

}