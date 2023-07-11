fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed: [[i32; 3]; 3] = [[0,0,0], [0,0,0], [0,0,0]];

    for (i, row) in matrix.into_iter().enumerate() {
        for (j, value) in row.into_iter().enumerate(){
            transposed[j][i] = value;
        }  
    }

    transposed
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!("{row:?}")
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
