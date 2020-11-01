use rand::Rng;

fn cmp<T: PartialOrd>(a: &mut T, b: &mut T) {
    if a > b {
        std::mem::swap(a, b);
    }
}

fn bitonic4<T: PartialOrd>(v: &mut [T; 4]) {
    let [ref mut v0, ref mut v1, ref mut v2, ref mut v3,] = v;
    cmp(v0, v1);
    cmp(v3, v2);
    cmp(v0, v2);
    cmp(v1, v3);
    cmp(v0, v1);
    cmp(v2, v3);
}

fn bitonic5<T: PartialOrd>(v: &mut [T; 5]) {
    let [ref mut v0, ref mut v1, ref mut v2, ref mut v3, ref mut v4] = v;
    cmp(v0, v1);
    cmp(v2, v3);
    cmp(v4, v2);
    cmp(v3, v2);
    cmp(v0, v4);
    cmp(v1, v3);
    cmp(v2, v4);
    cmp(v1, v2);
    cmp(v3, v4);
}

#[test]
fn test_bitonic5() {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let mut v: [u8; 5] = [0; 5];
        for i in 0..5 {
            v[i] = rng.gen();
        }
        bitonic5(&mut v);
        for i in 0..4 {
            assert!(v[i] <= v[i + 1]);
        }
    }
}

fn main() {
    let mut a: [i32; 4] = [3, 7, 2, 1];
    let mut b: [i32; 5] = [3, 7, 2, 1, 4];
    bitonic4(&mut a);
    bitonic5(&mut b);
    println!("{:?}", a);
    println!("{:?}", b);
}
