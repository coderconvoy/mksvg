extern crate num;
pub mod text;
pub mod cdnum;
pub mod page;

use std::io::Write;
use std::fmt::Display;
use cdnum::CDNum;
use num::NumCast;

pub fn qcast<A:CDNum,B:CDNum>(a:A)->B{
    NumCast::from(a).unwrap()
}

pub struct SvgW<W:Write> {
    w:W,
    d:i8,
}


impl<W:Write> SvgW<W> {
    pub fn new(w:W)->SvgW<W>{
        SvgW{
            w:w,
            d:0,
        }
    }
    fn pad(&self)->String{
        let mut res = "".to_string();
        for _i in 0..self.d {
            res.push_str("  ");
        }
        res
    }
}

impl<W:Write> Svg for SvgW<W> {
    fn write(&mut self, s:&str){
        let ps = self.pad();
        write!(self.w,"{}{}\n",ps,s).unwrap();
    }
    fn inc_depth(&mut self,n:i8){
        self.d += n;
    }
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


pub trait Svg {
    fn write(&mut self,s:&str);
    fn inc_depth(&mut self,n:i8);

    fn start<T:CDNum>(&mut self,w:T,h:T){
        self.write("<?xml version=\"1.0\" ?>");
        self.write(&format!(
               "<svg width={} height={}
    xmlns=\"http://www.w3.org/2000/svg\"
    xmlns:xlink=\"http://www.w3.org/1999/xlink\">",q(w),q(h),
               ));
        self.inc_depth(1);  
    }

    fn end(&mut self){
        self.inc_depth(-1);
        self.write("</svg>");
    }

    fn g_translate<T:CDNum>(&mut self, x:T,y:T,args:&str){
        self.write(&format!(
            "<g transform=\"translate({},{}) {}\">\n",x,y,args,
        ));
        self.inc_depth(1);
    }

    fn g(&mut self,args:&str ){
        self.write(&format!( "<g {}>\n",args));
        self.inc_depth(1);
    }

    fn g_end(&mut self){
        self.inc_depth(-1);
        self.write("</g>");
    }

    fn any(&mut self,name:&str,args:&str){
        self.write(&format!("<{} {} />",name,args));
    }
    

    fn rect<T:CDNum>(&mut self,x:T,y:T,w:T,h:T,args:&str){
        self.any("rect",&format!("x={} y={} width={} height={} {}",q(x),q(y),q(w),q(h),args));
    }

    fn text<T:CDNum>(&mut self,tx:&str,x:T,y:T,fs:T,args:&str,styles:&[&str]){
        
        let mut sty = st("font-size",fs);
        for s in styles {
            sty.push_str(s);
        }
        self.write(&format!(
                "<text x={} y={} {} {}>{}</text>",q(x),q(y),args,
                style(&[&sty]),tx,
        ));
    }

    fn text_lines<T:CDNum>(&mut self,tx:&str,x:T,y:T,fs:T,dy:T,args:&str,styles:&[&str]){
        let lns = tx.split("\n"); 
        let mut ln_y:T = y;
        for ln in lns{
            self.text(&ln,x,ln_y,fs,args,styles);
            ln_y = ln_y+  dy;
        }
    }

    fn img<T:CDNum>(&mut self,loc:&str,x:T,y:T,w:T,h:T){
        self.any("image",&format!("x={} y={} width={} height={} xlink:href={}",q(x),q(y),q(w),q(h),q(loc)));
    }

}




#[cfg(test)]
mod tests {
    use {SvgW,Svg,style,st};
    use std::str;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn maker() {
        let v = Vec::new();
        let mut s = SvgW::new(v);
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
