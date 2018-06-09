pub mod text;
pub mod cdnum;
use std::io::Write;
use std::fmt::Display;
use cdnum::CDNum;


pub struct Svg<W:Write> {
    w:W,
    d:u8,
}

//quoted
pub fn q<T:Display>(t:T)->String{
    format!("\"{}\"",t) 
}

pub fn st<T:Display>(s:&str,v:T)->String{
    format!("{}:{};",s,v) 
}

//to be used in conjuction with fn st()
//style(&[st(p,v),st(p,v) ...])
pub fn style(args:&[&str])->String{
    let mut res = "style=\"".to_string();
    for a in args{
        res.push_str(a)
    }
    res.push('\"');
    res
}

fn pad(d:u8)->String{
    let mut res = "".to_string();
    for _i in 0..d {
        res.push_str("  ");
    }
    res
}

impl<W:Write> Svg<W> {
    pub fn new(w:W)->Svg<W>{
        Svg{
            w:w,
            d:0,
        }
    }

    pub fn start<T:CDNum>(&mut self,w:T,h:T){
        write!(self.w,"<?xml version=\"1.0\" ?>\n").unwrap();
        write!(self.w,
               "<svg width={} height={}
    xmlns=\"http://www.w3.org/2000/svg\"
    xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n",q(w),q(h),
               ).unwrap();
        self.d += 1;
    }

    pub fn end(&mut self){
        self.d -= 1;
        write!(self.w,"</svg>\n").unwrap();
    }

    pub fn g_translate<T:CDNum>(&mut self, x:T,y:T,args:&str){
        write!(self.w,"{}<g transform=\"translate({},{}) {}\">\n",pad(self.d),x,y,args).unwrap();
        self.d += 1;
    }

    pub fn g(&mut self,args:&str ){
        write!(self.w,"{}<g {}>\n",pad(self.d),args).unwrap();
        self.d += 1;
    }

    pub fn g_end(&mut self){
        self.d -=1;
        write!(self.w,"{}</g>\n",pad(self.d)).unwrap();
    }

    pub fn any(&mut self,name:&str,args:&str){
        write!(self.w,"{}<{} {} />\n",pad(self.d),name,args).unwrap();
    }
    
    pub fn anyopen(&mut self,name:&str,args:&str){
        write!(self.w,"{}<{} {} >",pad(self.d),name,args).unwrap();
    }


    pub fn rect<T:CDNum>(&mut self,x:T,y:T,w:T,h:T,args:&str){
        self.any("rect",&format!("x={} y={} width={} height={} {}",q(x),q(y),q(w),q(h),args));
    }

    pub fn text<T:CDNum>(&mut self,tx:&str,x:T,y:T,fs:T,args:&str,styles:&[&str]){
        
        let mut sty = st("font-size",fs);
        for s in styles {
            sty.push_str(s);
        }
        self.anyopen("text",&format!("x={} y={} {} {}",q(x),q(y),args,style(&[&sty])));
        write!(self.w,"{}</text>",tx).unwrap();
    }

    pub fn lines<T:CDNum>(&mut self,tx:&str,x:T,y:T,fs:T,dy:T,args:&str,styles:&[&str]){
        let lns = tx.split("\n"); 
        let mut ln_y:T = y;
        for ln in lns{
            self.text(&ln,x,ln_y,fs,args,styles);
            ln_y = ln_y+  dy;
        }
    }

}




#[cfg(test)]
mod tests {
    use {Svg,style,st};
    use std::str;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn maker() {
        let v = Vec::new();
        let mut s = Svg::new(v);
        s.start(45 ,34 ); 
        let res = match str::from_utf8(&s.w){
            Ok(r)=>r,
            _=>"NoConv",
        };
        print!("{}",res);
        assert!(s.w.len() > 0);
    }

    #[test]
    fn style_t(){
        assert_eq!(style(&[&st("num",3),&st("st","hello")]),"style=\"num:3;st:hello;\"");
    }

}
