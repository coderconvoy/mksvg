
pub fn escapes(s:&str)->String{
    let mut res = String::new();
    let mut esc = false;
    for c in s.chars(){
        if esc {
            match c {
                't' => res.push('\t'),
                'n' => res.push('\n'),
                'r' => res.push('\r'),
                _=>res.push(c),
            }
            esc = false;
            continue;
        }
        match c {
            '\\' => esc = true,
            _ => res.push(c)  
        }
    }
    res
}

pub fn wrap_nl(s:&str, mx:usize)->String{
    wrap(s,mx).join("\n")
}

pub fn wrap(s:&str,mx:usize)->Vec<String>{
    let mut cword = String::new();
    let mut cline = String::new(); 
    let mut res:Vec<String> = Vec::new();
    
    for c in s.chars(){
        if cline.len() + cword.len() > mx {
            if cline.len() == 0 {
                cline.push_str(&cword[..mx]);
                cline.push('-');
                cword = String::from(&cword[mx..]);
            }else {
                cword = cword[1..].to_string();
            }

            res.push(cline);
            cline = "".to_string();
        }
        match c { 
            '\n'=>{ 
                cline.push_str(&cword);
                cword.clear();
                res.push(cline);
                cline = "".to_string();
            }
            '-'=>{
                cline.push_str(&cword);
                cline.push('-');
                cword = String::from(" ");
                
            }
            ' '=>{
                cline.push_str(&cword);
                cword= String::from(" ");
            }
            _=>{
                cword.push(c);
            }
        }
    }
    cline.push_str(&cword);
    res.push(cline);
    res
}

#[cfg(test)]
mod tests {
    use text::*;

    #[test]
    fn test_escapes(){
        assert_eq!(&escapes("he\\\\n \\n\\t\\p") ,"he\\n \n\tp");
    }

    #[test]
    fn test_wrap(){
        assert_eq!(&wrap("hello everybody",6),&["hello","everyb-","ody"]);
        assert_eq!(&wrap("hi to the people i know",6),&["hi to","the","people","i know"]);
    }
    #[test]
    fn test_wrap2(){
        assert_eq!(&wrap("he-llo hello-people",5),&["he-","llo","hello-","people"]);
    }
    
}
