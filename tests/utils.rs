extern crate proton_runner;


#[test]
fn transpose_data_works_with_valid_square_data() {
    let data = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];

    let correct_transposed = vec![
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![3, 6, 9]
    ];

    let transposed = proton_runner::utils::transpose_data(data).unwrap();

    assert_eq!(transposed, correct_transposed);
}

#[test]
fn transpose_data_works_with_valid_rectangle_data() {
    let data = vec![
        vec![1, 2, 3],
        vec![4, 5, 6]
    ];

    let correct_transposed = vec![
        vec![1, 4],
        vec![2, 5],
        vec![3, 6]
    ];

    let transposed = proton_runner::utils::transpose_data(data).unwrap();

    assert_eq!(transposed, correct_transposed);
}

#[test]
#[should_panic(expected = "EmptyData")]
fn transpose_data_fails_with_empty_vec() {
    let data = vec![];

    let _ = proton_runner::utils::transpose_data(data).unwrap();
}

#[test]
#[should_panic(expected = "EmptyData")]
fn transpose_data_fails_with_empty_inner_vec() {
    let data = vec![
        vec![],
        vec![]
    ];

    let _ = proton_runner::utils::transpose_data(data).unwrap();
}
