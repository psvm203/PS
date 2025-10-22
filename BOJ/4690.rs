fn main() {
    for a in 2..101 {
        for b in 2..101 {
            for c in b + 1..101 {
                for d in c + 1..101 {
                    let left = a * a * a;
                    let right = b * b * b + c * c * c + d * d * d;

                    if left == right {
                        println!("Cube = {a}, Triple = ({b},{c},{d})");
                        break;
                    } else if left < right {
                        break;
                    }
                }
            }
        }
    }
}
