
pub fn wrap(s:&str,mx:usize)->Vec<String>{
    let mut cword = "".to_string();
    let mut cline = "".to_string(); 
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
    use text::wrap;
    #[test]
    fn test_wrap(){
        assert_eq!(&wrap("hello everybody",6),&["hello","everyb-","ody"]);
    }
    #[test]
    fn test_wrap2(){
        assert_eq!(&wrap("hi to the people i know",6),&["hi to","the","people","i know"]);
    }
}
