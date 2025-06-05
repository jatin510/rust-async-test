
trait Component {
    fn search(&self, keyword: &str);
}

struct File {
    name: &'static str
}

impl File {
    pub fn new(name: &'static str)-> Self {
        File {
            name
        }
    }
}
impl Component for File {
    fn search(&self, keyword: &str) {
        println!("File name: {}, searching for keyword {:?}", self.name, keyword);
    }
}

struct Folder {
    name: &'static str,
    components: Vec<Box<dyn Component>>
}

impl Folder {
    pub fn new(name: &'static str) -> Self {
        Folder{
            name,
            components: Vec::new()
        }
    }

    pub fn add(&mut self, component: impl Component+ 'static){
        self.components.push(Box::new(component));
    }
}



impl Component for Folder {
    fn search(&self, keyword: &str) {
        println!(
            "Searching recursively for keyword {} in folder {}",
            keyword, self.name
        );

        for component in &self.components {
            component.search(keyword);
        }
    }

}


pub fn run_composite_pattern(){
    let file1 = File::new("File 1");
    let file2 = File::new("File 2");
    let file3 = File::new("File 3");

    let mut folder1 = Folder::new("Folder 1");
    folder1.add(file1);

    let mut folder2 = Folder::new("Folder 2");
    folder2.add(file2);
    folder2.add(file3);
    folder2.add(folder1);

    folder2.search("rose");
}