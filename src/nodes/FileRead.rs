use std::fs::File;

fn readFileData(filename: String) {
    let mut file = File::open("foo.txt");
    let mut contents = String::mew();
    file.read_to_string(&mut content);
    contents
}

fn createFile(filename: String, data: String) {}

fn deleteFile(filename: String, data: String) {}
