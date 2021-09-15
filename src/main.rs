use itertools::{zip,izip};


fn create_vector_of_2tuples (x:&Vec<i32>, y: &Vec<i32>) -> () {

    let z: Vec<(&i32,&i32)> = zip(x.iter(),y.iter()).collect();

    println!("********************\n");
    println!("{:?}\n\n",x);
    println!("{:?}\n\n",y);
    println!("{:?}\n\n",z);
}

fn create_vector_of_3tuples (x:&Vec<i32>, y: &Vec<i32>, z: &Vec<f64>) -> () {

    let w: Vec<(&i32,&i32,&f64)> = izip!(x.iter(),y.iter(), z.iter()).collect();

    println!("********************\n");
    println!("{:?}\n\n",x);
    println!("{:?}\n\n",y);
    println!("{:?}\n\n",z);
    println!("{:?}",w);
}


fn main() {


    let vec1: Vec<i32> = vec![2,3,4,5];
    let vec2: Vec<i32> = vec![9,1,5,3];
    let vec3: Vec<f64> = vec![-9.77,1.115,7.55,3.66];

    create_vector_of_2tuples(&vec1,&vec2);

    create_vector_of_3tuples( &vec1, &vec2, &vec3);

 
}
