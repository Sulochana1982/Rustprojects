fn main() {

    let mut a = [5, 67, 56, 34, 23, 98, 54, 76, 99];
    let mut temp = [0, 0, 0, 0, 0, 0, 0, 0, 0,];

    let n = a.len();

    let mut swap = false;
    


    for i in 0..n {

        for j in 0..n-1{

            if a[j] > a[j+1]{
                let b = a[j];
                a[j] = a[j+1];
                a[j+1] = b;
                swap = true;
            
            }
           
        }
           

            if swap == false {
                break;
            }
    }

    let mut  b = false;
    let mut big = n-1;
        
      let mut small = 0;
      
    
    for i in 0..n{

        if b == false {
            temp[i] = a[big];
            big=big-1;
            
        }
        
        else {
            temp[i] = a[small];
            small=small+1;
        }

        b=!b;
           
        
    }
      

 //println!("{:?}", temp)

 for i in 0..n
 {
    a[i] = temp[i];
  
 }
 //println!("{:?}",a);

}