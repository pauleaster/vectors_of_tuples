

use itertools::{zip,izip};




fn create_iterator_for_vector_of_2tuples <'a> (x:& 'a Vec<i32>, y: & 'a Vec<i32>)
                ->  std::iter::Zip<std::slice::Iter<'a, i32>, std::slice::Iter<'a, i32>> {

    
    zip(x.iter(),y.iter())
}

fn create_iterator_for_vector_of_2tuples_2 <'a> (x:& 'a Vec<i32>, y: & 'a Vec<i32>) 
            -> impl Iterator< Item=(& 'a i32, & 'a i32)> {

    
    zip(x.iter(),y.iter())
}


fn create_vector_of_2tuples_1<'a>(x: &'a Vec<i32>, y: &'a Vec<i32>) -> Vec<(&'a i32,&'a i32)> {


    let vec3: Vec<(&i32,&i32)> =  zip(x.iter(),y.iter()).collect();

    vec3

}

fn create_vector_of_2tuples_2<'a>(x: &'a Vec<i32>, y: &'a Vec<i32>) -> Vec<(&'a i32,&'a i32)> {


    zip(x.iter(),y.iter()).collect()

}

fn create_vector_of_2tuples_3<'a>(x: &'a Vec<i32>, y: &'a Vec<i32>) -> Vec<(&'a i32,&'a i32)> {



    
    create_iterator_for_vector_of_2tuples(x,y).collect()

}

fn create_vector_of_2tuples_4<'a>(x: &'a Vec<i32>, y: &'a Vec<i32>) -> Vec<(&'a i32,&'a i32)> {



    
    create_iterator_for_vector_of_2tuples_2(x,y).collect()

}


fn create_iterator_for_vector_of_3tuples <'a> (x:& 'a Vec<i32>, y: & 'a Vec<i32>, z: &'a Vec<f64>)  
-> impl Iterator< Item=(& 'a i32, & 'a i32, & 'a f64)> {
 
    
    izip!(x.iter(),y.iter(), z.iter())
}


fn create_vector_of_3tuples<'a>(x: &'a Vec<i32>, y: &'a Vec<i32>, z: &'a Vec<f64>) -> Vec<(&'a i32,&'a i32, &'a f64)> {



    
    create_iterator_for_vector_of_3tuples(x,y,z).collect()

}



fn print_vector_of_3tuples (x:&Vec<i32>, y: &Vec<i32>, z: &Vec<f64>) -> () {

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

    let vec4 = create_vector_of_2tuples_3(&vec1,&vec2);
    let vec5 = create_vector_of_2tuples_4(&vec1,&vec2);
    let vec6 = create_vector_of_3tuples(&vec1,&vec2, &vec3);

    print_vector_of_3tuples( &vec1, &vec2, &vec3);

    println!("\n\n{:?}", vec4);
    println!("\n\n{:?}", vec5);
    println!("\n\n{:?}", vec6);

 
}
