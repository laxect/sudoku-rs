pub struct Board {
    inner: Vec<u8>,
}

impl Board {
    fn new () -> Board {
        Board {
            inner: vec![0; 81];
        }
    }
}
