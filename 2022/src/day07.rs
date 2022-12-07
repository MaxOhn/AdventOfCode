use std::{
    cell::RefCell,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Add,
    rc::{Rc, Weak},
    str::Lines,
};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let fs = FileSystem::parse(input)?;
    // println!("{fs}");

    let map = |entry: &DirEntry| match entry {
        DirEntry::Dir(dir) => {
            let size = dir.borrow().size;

            if size <= 100_000 {
                size
            } else {
                0
            }
        }
        DirEntry::File(_) => 0,
    };

    let reduce = <usize as Add>::add;

    let p1 = fs.map_reduce(map, reduce);

    const TOTAL_MEM: usize = 70_000_000;
    const REQUIRED_MEM: usize = 30_000_000;
    const FREE_MEM: usize = TOTAL_MEM - REQUIRED_MEM;
    let to_delete = fs.size() - FREE_MEM;

    let p2 = fs.dir_min_by_key(|dir| {
        if dir.size < to_delete {
            usize::MAX
        } else {
            dir.size
        }
    });

    Ok(Solution::new().part1(p1).part2(p2))
}

struct FileSystem<'i> {
    root: Dir<'i>,
}

impl<'i> FileSystem<'i> {
    fn parse(input: &'i str) -> Result<Self> {
        let mut lines = input.lines();

        let this = Self {
            root: Rc::new(RefCell::new(Directory::new("/"))),
        };

        let mut dir = Rc::clone(&this.root);

        while let Some(next) = read_command(dir, &mut lines)? {
            dir = next;
        }

        this.finalize_dir_sizes();

        Ok(this)
    }

    fn finalize_dir_sizes(&self) {
        self.root.borrow_mut().finalize_size()
    }

    fn size(&self) -> usize {
        self.root.borrow().size
    }

    fn map_reduce<M, O, F>(&self, map: M, fold: F) -> O
    where
        M: Copy + Fn(&DirEntry) -> O,
        F: Copy + Fn(O, O) -> O,
    {
        DirEntry::Dir(Rc::clone(&self.root)).map_reduce(map, fold)
    }

    fn dir_min_by_key<F, V>(&self, f: F) -> V
    where
        F: Copy + Fn(&Directory) -> V,
        V: Ord,
    {
        self.root.borrow().min_by_key(f)
    }
}

impl Display for FileSystem<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.root.borrow().print(f, 0)
    }
}

fn read_command<'i>(curr: Dir<'i>, lines: &mut Lines<'i>) -> Result<Option<Dir<'i>>> {
    match lines.next().map(Command::parse) {
        Some(Ok(Command::Cd(cmd))) => process_cd(cmd, curr),
        Some(Ok(Command::Ls)) => {
            for line in &mut *lines {
                if let Ok(entry) = DirEntry::parse(line) {
                    if let DirEntry::Dir(ref dir) = entry {
                        dir.borrow_mut().parent = Some(Rc::downgrade(&curr));
                    }

                    curr.borrow_mut().entries.push(entry);
                } else if let Ok(cmd) = Command::parse(line) {
                    match cmd {
                        Command::Cd(cmd) => return process_cd(cmd, curr),
                        Command::Ls => {}
                    }
                } else {
                    bail!("line `{line}` is neither dir entry nor command");
                }
            }

            Ok(None)
        }
        Some(Err(err)) => Err(err),
        None => Ok(None),
    }
}

fn process_cd<'i>(cmd: CdCommand<'i>, curr: Dir<'i>) -> Result<Option<Dir<'i>>> {
    match cmd {
        CdCommand::Root => Ok(Some(Directory::root(curr))),
        CdCommand::StepOut => Ok(curr.borrow().parent.as_ref().and_then(Weak::upgrade)),
        CdCommand::StepInto { dir } => curr
            .borrow()
            .entries
            .iter()
            .find_map(|entry| match entry {
                DirEntry::Dir(dir_entry) => (dir_entry.borrow().name == dir).then_some(dir_entry),
                DirEntry::File(_) => None,
            })
            .map(Rc::clone)
            .ok_or_else(|| {
                let parent = curr.borrow().name;

                eyre!("directory `{parent}` contains no directory `{dir}`")
            })
            .map(Some),
    }
}

enum Command<'d> {
    Cd(CdCommand<'d>),
    Ls,
}

impl<'d> Command<'d> {
    fn parse(cmd: &'d str) -> Result<Self> {
        let cmd = cmd.strip_prefix("$ ").wrap_err("missing command prefix")?;

        if cmd.starts_with("ls") {
            Ok(Self::Ls)
        } else if let Some(suffix) = cmd.strip_prefix("cd ") {
            Ok(Self::Cd(CdCommand::parse(suffix)))
        } else {
            bail!("invalid command `{cmd}`")
        }
    }
}

enum CdCommand<'d> {
    Root,
    StepOut,
    StepInto { dir: &'d str },
}

impl<'d> CdCommand<'d> {
    fn parse(s: &'d str) -> Self {
        match s {
            ".." => Self::StepOut,
            "/" => Self::Root,
            _ => Self::StepInto { dir: s },
        }
    }
}

enum DirEntry<'i> {
    Dir(Dir<'i>),
    File(File<'i>),
}

impl<'i> DirEntry<'i> {
    fn parse(s: &'i str) -> Result<Self> {
        let (prefix, suffix) = s.split_once(' ').wrap_err("invalid dir entry")?;

        if prefix == "dir" {
            Ok(Self::Dir(Rc::new(RefCell::new(Directory::new(suffix)))))
        } else if let Ok(size) = prefix.parse() {
            Ok(Self::File(File { name: suffix, size }))
        } else {
            bail!("invalid pefix `{prefix}` for dir entry");
        }
    }

    fn size(&self) -> usize {
        match self {
            DirEntry::Dir(dir) => dir.borrow().size,
            DirEntry::File(file) => file.size,
        }
    }

    fn map_reduce<M, O, F>(&self, map: M, fold: F) -> O
    where
        M: Copy + Fn(&Self) -> O,
        F: Copy + Fn(O, O) -> O,
    {
        match self {
            DirEntry::Dir(dir) => dir
                .borrow()
                .entries
                .iter()
                .map(|entry| entry.map_reduce(map, fold))
                .fold(map(self), fold),
            DirEntry::File(_) => map(self),
        }
    }
}

type Dir<'i> = Rc<RefCell<Directory<'i>>>;

struct Directory<'n> {
    name: &'n str,
    entries: Vec<DirEntry<'n>>,
    parent: Option<Weak<RefCell<Directory<'n>>>>,
    size: usize,
}

impl<'n> Directory<'n> {
    fn new(name: &'n str) -> Self {
        Self {
            name,
            entries: Vec::new(),
            parent: None,
            size: 0,
        }
    }

    fn finalize_size(&mut self) {
        fn size(dir: &Directory) -> usize {
            dir.entries.iter().map(DirEntry::size).sum()
        }

        for entry in self.entries.iter() {
            if let DirEntry::Dir(ref dir) = entry {
                dir.borrow_mut().finalize_size();
            }
        }

        self.size = size(self);
    }

    fn root(curr: Dir) -> Dir {
        if let Some(parent) = curr.borrow().parent.as_ref().and_then(Weak::upgrade) {
            return parent;
        }

        curr
    }

    fn print(&self, f: &mut Formatter<'_>, mut indent: usize) -> FmtResult {
        writeln!(
            f,
            "{space}{dash:>indent$} {name} (dir)",
            space = if indent == 0 { "" } else { " " },
            dash = "-",
            name = self.name
        )?;

        indent += 2;

        for entry in self.entries.iter() {
            match entry {
                DirEntry::Dir(next) => next.borrow().print(f, indent)?,
                DirEntry::File(file) => writeln!(
                    f,
                    " {dash:>indent$} {name} (file, size={size})",
                    dash = "-",
                    name = file.name,
                    size = file.size,
                )?,
            }
        }

        Ok(())
    }

    fn min_by_key<F, V>(&self, f: F) -> V
    where
        F: Copy + Fn(&Self) -> V,
        V: Ord,
    {
        self.entries
            .iter()
            .filter_map(|entry| match entry {
                DirEntry::Dir(ref dir) => Some(dir.borrow().min_by_key(f)),
                DirEntry::File(_) => None,
            })
            .min()
            .map_or_else(|| f(self), |min| f(self).min(min))
    }
}

struct File<'n> {
    name: &'n str,
    size: usize,
}
