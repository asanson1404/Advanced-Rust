macro_rules! cartesian {

    // First rule : take 2 expressions in parameter and return all the possible 
    // combinaisons from the 2 iterators (right expression vary faster)
    ($left:expr, $right:expr) => {
        $left.into_iter().flat_map(|l| $right.into_iter().map(move |r| (l.clone(), r)))
    };

    // Second rule : same as first rule but takes 3 expressions
    // Left expressions vary slower than that of right
    ($left:expr, $middle:expr, $right:expr) => {
        cartesian!($left, cartesian!($middle, $right)).map(|(a, (b, c))| (a, b, c))
    };
}

fn main() {

    let test_r1 = cartesian!(
        [1, 2, 3],
        [String::from("foo"), String::from("bar")]
      ).collect::<Vec<_>>();

    println!("{test_r1:?}");

    let test_r2 = cartesian!(
        [1, 2, 3],
        [String::from("foo"), String::from("bar")],
        ['A', 'B', 'C']
      ).collect::<Vec<_>>();

    println!("{test_r2:?}");
}
