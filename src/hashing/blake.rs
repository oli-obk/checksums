use self::super::hash_string;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use blake::Blake;


pub fn hash(path: &PathBuf) -> String {
    let mut file = File::open(path).unwrap();
    let mut buffer = vec![0; 1024];

    let blake = Blake::new(512).unwrap();
    loop {
        let read = file.read(&mut buffer[..]).unwrap();

        if read == 0 {
            break;
        }

        blake.update(&buffer[0..read]);
    }

    let mut result = [0; 64];
    blake.finalise(&mut result);
    hash_string(&result)
}
