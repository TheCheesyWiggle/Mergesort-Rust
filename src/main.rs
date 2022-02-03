//imports crate
extern crate rand;

fn main() {
    //sets up array to sort
    let array : &mut [i32] = &mut [];
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
//-> &mut [i32] sets function type to &mut [i32]
fn merge_sort(array: &mut [i32]) -> &mut [i32] {
    //sets the middle
    let mid = array.len()/2;
    //sets up left and right array
    let &mut left = [mid..];
    let &mut right = [..mid];
    //checks if the array is larger than 2
    if array.len()<=2{
        //returns array if there is one/no values in it
        return array;
    }
    //recurses through
    merge_sort(left);
    merge_sort(right);
    //merges all the lists and returns sorted list
    return merge(left,right);
}
//merging the lists
fn merge <'a>(left: &'a mut [i32],right : &'a mut [i32]) -> &'a mut [i32]{
    //creates empty array
    let mergedList : &mut [i32] = &mut [];
    //sets pointers for the lists
    let mut indexLeft = 0;
    let mut indexRight = 0;
    let mut indexMerged = 0;
    //while pointers are not at the end of the list it will loop
    while(indexLeft < left.len())||(indexRight < right.len()){
        //checks if both arrays still have unvisited values
        if(indexLeft < left.len())&&(indexRight < right.len()) {
            //checks if the right number is higher
            if left[indexLeft] <= right[indexRight] {
                //adds the right lists value to the merged array
                mergedList[indexRight] = right[indexRight];
                //increases pointers
                indexRight = indexRight +1;
                indexMerged = indexMerged +1;
            }
            else {
                //adds the left lists value to the merged array
                mergedList[indexLeft] = right[indexLeft];
                //increases pointers
                indexLeft = indexLeft +1;
                indexMerged = indexMerged +1;
            }
        }
        //adds remaining left values
        else if indexLeft < left.len() {
            //adds the left lists value to the merged array
            mergedList[indexLeft] = right[indexLeft];
            //increases pointers
            indexLeft = indexLeft +1;
            indexMerged = indexMerged +1;
        }
            //adds remaining right values
        else if indexRight < right.len() {
            //adds the right lists value to the merged array
            mergedList[indexRight] = right[indexRight];
            //increases pointers
            indexRight = indexRight +1;
            indexMerged = indexMerged +1;
        }
    }
    //returns sorted list
    return mergedList;
}