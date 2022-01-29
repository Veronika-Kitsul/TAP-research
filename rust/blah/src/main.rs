fn main() {
    let s = "3
2
1 2
10
3
3 1 2
111
8
2 3 1 8 5 4 7 6
01110001
";

    let chunks: Vec<_> = s.split_whitespace().collect();

    println!("{:?}", chunks);
}

