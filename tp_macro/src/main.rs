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

macro_rules! forth {
    // add followed by a rest 
    ($stack:expr, add; $($rest:tt)+) => {
        {
            let a = $stack.pop().unwrap();
            let b = $stack.pop().unwrap();
            $stack.push(a + b);
            forth!($stack, $($rest)+)
        }
    };
    // add at the end
    ($stack:expr, add$(;)?) => {
        {
            let b = $stack.pop().unwrap();
            let a = $stack.pop().unwrap();
            $stack.push(a + b);
            $stack
        }
    };
    // dup followed by a rest
    ($stack:expr, dup; $($rest:tt)+) => {
        {
            let a = $stack.last().unwrap();
            $stack.push(*a);
            forth!($stack, $($rest)+)
        }
    };
    // dup at the end
    ($stack:expr, dup$(;)?) => {
        {
            let a = $stack.last().unwrap();
            $stack.push(*a);
            $stack
        }
    };
    // mul followed by a rest 
    ($stack:expr, mul; $($rest:tt)+) => {
        {
            let a = $stack.pop().unwrap();
            let b = $stack.pop().unwrap();
            $stack.push(a * b);
            forth!($stack, $($rest)+)
        }
    };
    // mul at the end
    ($stack:expr, mul$(;)?) => {
        {
            let b = $stack.pop().unwrap();
            let a = $stack.pop().unwrap();
            $stack.push(a * b);
            $stack
        }
    };
    // First integer
    ($val:expr$(;)?) => {
        vec!($val)
    };
    // First integer followed by a rest
    ($val:expr; $($rest:tt)+) => {
        {
            let mut stack = forth!($val);
            forth! (stack, $($rest)+)
        }
    };
    // The Stack followed by an integer and a rest
    ($stack:expr, $val:expr; $($rest:tt)+) => {
        {
            $stack.push($val);
            forth! ($stack, $($rest)+)
        }
    };
    // Integer at the end
    ($stack:expr, $val:expr$(;)?)=> {
        {
            $stack.push($val);
            $stack
        }
    };
}

trait ZeroArith {
    fn zero_add(self, other: Self) -> Self;
    fn zero_sub(self, other: Self) -> Self;
    fn zero_mul(self, other: Self) -> Self;
    fn zero_div(self, other: Self) -> Self;
}

// Macro to not repeat the match when we implement the trait ZeroArith
macro_rules! check_match {
    ($to_match:expr$(;)?) => {
        match $to_match {
            Some(res) => res,
            None      => 0
        }
    };
}

macro_rules! impl_zero_arith {
    ($($type:ty),+$(,)?) => {
        $(impl ZeroArith for $type {
            fn zero_add(self, other: Self) -> Self {
                check_match!(self.checked_add(other))
            }
            fn zero_sub(self, other: Self) -> Self {
                check_match!(self.checked_sub(other))
            }
            fn zero_mul(self, other: Self) -> Self {
                check_match!(self.checked_mul(other))
            }
            fn zero_div(self, other: Self) -> Self {
                check_match!(self.checked_div(other))
            }
        })+
    }
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
    /*debug!();
    println!("Result = {}", 10 + debug!(2*3));
    debug!(println!("foobar"));*/

    // Test macro forth!()
    // Intermediate test
    /*println!("final stack = {:?}", forth!(10));
    println!("final stack = {:?}", forth!(10; 20));
    println!("final stack = {:?}", forth!(10; 20; 30));
    println!("final stack = {:?}", forth!(10; 20; 30; add; 40));
    println!("final stack = {:?}", forth!(10; 20; 30; add; 40; add));
    println!("final stack = {:?}", forth!(10; 20; 30; dup; 40; dup));
    println!("final stack = {:?}", forth!(10; 20; 30; mul; 40; mul));
    // Final test
    println!("final stack = {:?}", forth!(10; 20; add; dup; 6; mul));*/

    // Test macro impl_zero_arith!()
    impl_zero_arith!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128,);
    println!("{}", 200u8.zero_add(200u8));
    println!("{}", 200u8.zero_add(50u8));
    println!("{}", 200u8.zero_sub(250u8));
    println!("{}", 200u8.zero_sub(150u8));
    println!("{}", 200u8.zero_div(0));

}
