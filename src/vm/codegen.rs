pub struct ByteCode {

}

pub trait Generator {
    fn gen(&mut self) -> ByteCode;
}
