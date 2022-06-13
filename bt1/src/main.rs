fn main() {
    let org_arr:[i32;8] = [1,2,3,5,6,8,10,11];
    let sub_arr:[i32;3] = [6,8,10]; 

    let check: bool = is_a_contain_b(&org_arr,&sub_arr);
    println!("{}",check);

  
}

fn is_a_contain_b(A: &[i32], B: &[i32]) -> bool{
 
    let (mut i, mut j):(i32,i32) = (0,0);
    let  n:i32 = A.len().try_into().unwrap();
    let  m:i32 = B.len().try_into().unwrap();
  //println!("{}", n);
    while i < n && j < m {
        if A[i as usize] == B[j as usize]{
              i += 1;
              j += 1;
            if j == m {
                return true;
            }
        }else {
            i = i + 1;
            j = 0;
        }
         
    }
   return false;
   
}


