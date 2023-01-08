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

macro_rules! debug {

    // If no parameters, just return the file and the line where the macro has been called
    () => {
        println!("{}:{}", file!(), line!())
    };
    
    // Display the file and the current line, the literal expression of the argument and its value
    ($val:expr $(,)?) => {{
        let eval = $val;
        println!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!($val), eval);
        eval
    }};
}

fn main() {

    // Test macro cartesian!()
    /*let test_r1 = cartesian!(
        [1, 2, 3],
        [String::from("foo"), String::from("bar")]
      ).collect::<Vec<_>>();

    println!("{test_r1:?}");

    let test_r2 = cartesian!(
        [1, 2, 3],
        [String::from("foo"), String::from("bar")],
        ['A', 'B', 'C']
      ).collect::<Vec<_>>();

    println!("{test_r2:?}");*/
    
    // Test macro debug!()
    debug!();
    println!("Result = {}", 10 + debug!(2*3));
    debug!(println!("foobar"));
}
