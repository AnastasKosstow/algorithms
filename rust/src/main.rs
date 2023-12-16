mod sorting;

fn main() 
{
    let mut vector: Vec<i32>;

    vector = vec![0,6,4,6,9,9,5,8,2,4,3,2,8,8,2];
    sorting::bubble_sort(&mut vector);
    
    vector = vec![9,6,3,9,6,1,9,9,7,0,5,0,7,4,3];
    sorting::selection_sort(&mut vector);

    vector = vec![5,5,3,0,9,5,1,6,0,9,0,7,2,2,9];
    sorting::insertion_sort(&mut vector);

    vector = vec![8,4,8,9,7,1,7,6,4,1,4,3,0,6,0];
    sorting::shell_sort(&mut vector);

    vector = vec![3,5,8,1,2,0,3,1,8,4,8,5,8,9,1];
    sorting::merge_sort(&mut vector);

}
