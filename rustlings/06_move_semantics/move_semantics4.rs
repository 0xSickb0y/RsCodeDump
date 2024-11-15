fn move_semantics4() {
    let mut x = Vec::new();
    assert_eq!(x, [42, 13]);

    let y = &mut x;
    y.push(42);
    
    let z = &mut x;
    z.push(13);
}

fn main() {
    // You can optionally experiment here.
    move_semantics4();
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        assert_eq!(x, [42, 13]);
        let y = &mut x;
        y.push(42);
        let z = &mut x;
        z.push(13);
    }
}
