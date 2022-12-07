use std::collections::HashMap;

struct FileSystem {
    // Full path -> list of subdirectories (also full paths)
    subdirs: HashMap<String, Vec<String>>,

    // Full path -> size of files (without subdirectories)
    files: HashMap<String, u64>,

    // Full path -> size of entire directory (with subdirectories)
    sizes: HashMap<String, u64>,

    // Current path
    cd: Vec<String>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            subdirs: HashMap::new(),
            sizes: HashMap::new(),
            files: HashMap::new(),
            cd: Vec::new(),
        }
    }

    fn total_size(&self) -> u64 {
        // NOTE: Taking the max here ensures this is generic regardless of what that root directory's name is
        *self.sizes.values().max().unwrap()
    }

    fn change_dir(&mut self, target: &str) {
        if target == ".." {
            self.cd.pop();
        } else {
            self.cd.push(target.to_string());
        }
    }

    fn register_subdir(&mut self, name: &str) {
        let this_dir = self.cd.join("/");
        let subdir = this_dir.clone() + "/" + name;

        self.subdirs
            .entry(this_dir)
            .or_insert_with(Vec::new)
            .push(subdir);
    }

    fn register_file(&mut self, size: u64) {
        let this_dir = self.cd.join("/");

        self.files
            .entry(this_dir)
            .and_modify(|s| *s += size)
            .or_insert(size);
    }

    fn accumulate_directory_sizes(&mut self) {
        for (dir, subdirs) in &self.subdirs {
            // If no files, size is 0
            let mut size = *self.files.get(dir).unwrap_or(&0);

            for subdir in subdirs {
                let subdir_size = self.dir_size(subdir);
                self.sizes.insert(subdir.clone(), subdir_size);

                size += subdir_size;
            }            
            
            self.sizes.insert(dir.clone(), size);
        }
    }
    
    // Recursively calculate the size of a directory
    fn dir_size(&self, dir: &String) -> u64 {
        // If no files, size is 0
        let mut total = *self.files.get(dir).unwrap_or(&0);

        if let Some(subdirs) = self.subdirs.get(dir) {
            for subdir in subdirs {
                let nested_size = self.dir_size(subdir);
                total += nested_size;
            }
        }

        total
    }
}

fn main() {
    let input = aoc2022::read_input_for_day(7);

    let mut fs = FileSystem::new();

    for line in input.lines() {
        if let Some(target) = line.strip_prefix("$ cd ") {
            fs.change_dir(target);
        }
        else if line == "$ ls" {
            // do nothing
        }
        else if let Some(subdir) = line.strip_prefix("dir ") {
            fs.register_subdir(subdir);
        }
        else {
            // Must be a file
            let size: u64 = line
                .split_once(' ')
                .unwrap()
                .0
                .parse()
                .unwrap();
            
            fs.register_file(size);
        }
    }

    // NOTE: There is probably a better solution that wouldn't need this as a separate step
    fs.accumulate_directory_sizes();

    let part1 = fs.sizes
        .iter()
        .filter_map(|(_dir, &size)| if size <= 100000 { Some(size) } else { None })
        .sum::<u64>();

    println!("Part 1: {}", part1);


    let free_space = 70000000 - fs.total_size();
    let part2 = fs.sizes
        .values()
        .filter(|&size| free_space + size >= 30000000)
        .min()
        .unwrap();

    println!("Part 2: {}", part2);
}