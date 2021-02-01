impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {

  let mut l: i32 = 0;
    let mut k: i32 = 0;
    let mut u: i32 = 0;
    let n=0;
    let m=0;
    let mut c =0;
    let mut o=0;
    let mut p=0;
    let g = a.len()-1;
    let f = b.len()-1;
    let mut d =0;
    let  mut t: Vec<i32>=Vec::new();
    
    for i in &a{
        l = l+i;
    }
    for h in &b{
        k = k+h;
    }
    if l>k{
        u = (l-k)/2;
    }else{
        u = (k-l)/2;
    }
    if l>k{
      loop{
            let n =&a[o];
            let m =&b[p];
             println!("{} {}",n,m);
              println!("{} {}",o,p);
            if (n-m)==u{
                println!("{} {}",n,m); 
                t.push(*n);
                t.push(*m);    //0 0 
                return break;              //0 1 
            }
            
            
        
        if p == f&&o!=g{
            
            o=o+1;
            
        }   
             if p==f{
          p =0;
          }
  if c ==0{
    p = p+1;
    c= c+1;
  }else{
   c = c-1;

  }
        
        
      
        }
    
    
    }else{
        loop{
        let n =&a[o];
        let m =&b[p];
        if (m-n)==u{
            println!("{} {}",n,m);
            t.push(*n);
                t.push(*m);
            return break;
        }
          
        if p == f&&o!=g{
            
            o=o+1;
            
        } 
             if p==f{
          p =0;
          }  
  if c ==0{
    p = p+1;
    c= c+1;
  }else{
   c = c-1;
   
  }
           
            
             
            
       }
    }
    t
    }


}
