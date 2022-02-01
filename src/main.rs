//imports crate
extern crate rand;

fn main() {
    //sets up array to sort
    let array : &mut [i32];
    //assigns random numbers to the array
    for z in array{
        array[z]== rand::random();
    }
    //outputs unsorted array
    for x in array {
        print!("{} ", x);
    }
    merge_sort(array);
}
//start of the mergesort
fn merge_sort( array: &mut [i32]){
    //sets the middle
    let mid = array.len()/2;
    //checks if there are any items in the array
    if mid ==0{
        return;
    }
    //sets up left and right array
    let &mut left = [mid..];
    let &mut right = [..mid];
    //recurses through
    merge_sort(left);
    merge_sort(right);

    merge(left,right)
}
fn merge(left: &mut [i32],right : &mut [i32]){

}