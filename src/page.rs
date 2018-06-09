use std::io::Write;
use Svg;
use {style,st}
use cdnum::CDNum;

pub trait Card<NT:CDNum> :Clone{
    front(&self,svg:&svg,w:NT,h:NT);
    back(&self)->Card<NT>{
        self.clone() 
    }
    
}

pub fn page_a4<W:Write,NT:CDNum,C:Card<NT>>(w:W,nw:usize,nh:usize,cards:&[C]){
    page(w,2480,3508,nw,nh,cards);
}

pub fn page<W:Write,NT:CDNum,C:Card<NT>>(w:W,pw:NT,ph:NT,nw:usize,nh:usize,cards:&[C]){
    let mut svg = Svg::new(w);
    svg.start(pw,ph);

    let mw = pw/20;
    let mh = ph/20;
    let max = nw * nh;
    let cw = (pw - 2*mw )/(nw as NT);
    let ch = (ph - 2*mh)/(nh as NT);


    for (i,c) in cards.iter().enumerate(){
        let x = (i % nw)  as NT;
        let y = (i % nh) as NT
        svg.g_translate(mw+ x*cw,mh+y*ch);
        c.front(&svg,cw,ch);
        svg.g_end();

    }

}

