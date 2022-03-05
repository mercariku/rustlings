// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

// I AM !NOT DONE

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}

// What it looks like when test fails
// running 1 test
// test slice_out_of_array ... FAILED

// successes:

// successes:

// failures:

// ---- slice_out_of_array stdout ----
// thread 'slice_out_of_array' panicked at 'assertion failed: `(left == right)`
//   left: `[2, 3, 4]`,
//  right: `[2, 3]`', exercises/primitive_types/primitive_types4.rs:13:5
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


// failures:
//     slice_out_of_array
