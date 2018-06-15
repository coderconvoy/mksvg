use std::io::Write;
use std::fs::File;
use {Svg,SvgW,qcast};
use cdnum::CDNum;


pub trait Card<NT:CDNum> :Clone{
    fn front<S:Svg>(&self,svg:&mut S,w:NT,h:NT);
}


pub fn page<W:Write,NT:CDNum,C:Card<NT>>(w:W,pw:NT,ph:NT,nw:usize,nh:usize,cards:&[C]){
    let mut svg = SvgW::new(w);
    svg.start(pw,ph);

    //let n_20:NT = NumCast::from(20).unwrap();
    //let n_2:NT = NumCast::from(2).unwrap();
    let mw:NT = pw/qcast(20);
    let mh:NT = ph/qcast(20);
    let max = nw * nh;
    let cw = (pw - qcast::<i32,NT>(2)*mw )/qcast(nw);
    let ch = (ph - qcast::<i32,NT>(2)*mh)/qcast(nh);


    for (i,c) in cards.iter().enumerate(){
        let x:NT = qcast(i % nw);
        let y:NT = qcast(i / nw);
        svg.g_translate(mw+ x*cw,mh+y*ch,"");
        c.front(&mut svg,cw,ch);
        svg.g_end();
        if i == max {
            break;
        }
    }

    svg.end();

}


pub fn page_a4<W:Write,NT:CDNum,C:Card<NT>>(w:W,nw:usize,nh:usize,cards:&[C]){
    page(w,qcast::<i32,NT>(2480),qcast::<i32,NT>(3508),nw,nh,cards);
}

pub fn pages<NT:CDNum,C:Card<NT>>(basepath:&str,pw:NT,ph:NT,nw:usize,nh:usize,cards:&[C])->Result<bool,String>{
    let total = nw * nh; 
    for i in 0 .. cards.len() %total {
        let mut fname = basepath.to_string();
        fname.push_str(&i.to_string());
        fname.push_str(".svg");
        let w = match File::create(&fname) {
            Ok(f)=>f,
            Err(_)=>{
                return Err(format!("could not create {}",fname));
            }
        };
        page(w,pw,ph,nw,nh,cards);//TODO work out which slice
    }
    Ok(true)
}


pub fn pages_a4<NT:CDNum,C:Card<NT>>(basepath:&str,nw:usize,nh:usize,cards:&[C])->Result<bool,String>{
    pages(basepath,qcast::<i32,NT>(2480),qcast::<i32,NT>(3508),nw,nh,cards)
}

pub fn page_flip<T:Clone>(v:&Vec<T>,w:usize)->Vec<T>{
    //TODO
    let mut res:Vec<T> = Vec::new();
    if v.len() == 0 {
        return res;
    }
    let blank = v[0].clone();
    let mut tmp = Vec::new();
    for elem in v {
        tmp.push(elem.clone());
        if tmp.len() == w {
            for e2 in tmp.into_iter().rev() {
                res.push(e2);
            }
            tmp = Vec::new();
        }
    }

    if tmp.len() > 0{
        for _ in 0 .. w - tmp.len(){
            res.push(blank.clone());
        }
        for elem in tmp {
            res.push(elem);
        }
    }
    res
}


#[cfg(test)]
mod tests {
    use page::page_flip;
    #[test]
    fn test_flip() {
        let v = vec![1,2,3,4,5,6,7,8,9];
        let v2 = page_flip(&v,4);
        assert_eq!(v2,vec![4,3,2,1,8,7,6,5,1,1,1,9]);
    }
}
