use std::io::Write;

pub struct Svg<W:Write> {
    w:W
}

impl<W:Write> Svg<W> {
    pub fn new(w:W)->Svg<W>{
        Svg{
            w:w,
        }
    }

    pub fn start(&mut self,w:i32,h:i32){
        write!(self.w,"<?xml version=\"1.0\" ?>\n").unwrap();
        write!(self.w,
               "<svg width=\"{}\" height=\"{}\"
  1      xmlns=\"http://www.w3.org/2000/svg\"
  2      xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n",w,h,
               ).unwrap();
    }

    pub fn end(&mut self){
        write!(self.w,"</svg>\n");
    }

    
}




#[cfg(test)]
mod tests {
    use Svg;
    use std::str;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn maker() {
        let v = Vec::new();
        let mut s = Svg::new(v);
        s.start(45,34); 
        let res = match str::from_utf8(&s.w){
            Ok(r)=>r,
            _=>"NoConv",
        };
        print!("{}",res);
        assert!(s.w.len() > 0);
    }

}
