fn reverse<T>(vec: &mut Vec<T>, a: usize, n: usize) {
    let b = a + n - 1;
    let l = vec.len();
    for i in 0..n/2 {
        vec.swap((a + i) % l, (b - i) % l);
    }
}

fn hash_step<I>(list: &mut Vec<u16>, input: I)
    where I: Iterator<Item = usize>
{
    let mut pos = 0;
    let len = list.len();

    for (skip, n) in input.enumerate() {
        reverse(list, pos, n);
        pos = (pos + n + skip) % len;
    }
}

pub fn simple_hash<I>(input: I, size: u16) -> u16
    where I: Iterator<Item = usize>
{
    let mut list = (0..size).collect();
    hash_step(&mut list, input);
    list[0] * list[1]
}

pub fn full_hash(input: &str, size: u16) -> String {
    let mut list = (0..size).collect();
    let mut input = input.bytes().map(|x| x as usize).collect::<Vec<_>>();
    input.extend_from_slice(&[17, 31, 73, 47, 23]);
    
    hash_step(&mut list, input.iter().cycle().take(64*input.len()).cloned());

    list.chunks(16)
        .map(|chunk| format!("{:02x}", chunk.iter().fold(0, |acc, &x| acc ^ x)))
        .collect::<Vec<_>>()
        .concat()
}

#[cfg(test)]
mod test {
    use ::knothash::*;
    
    #[test]
    fn test_simple_hash() {
        assert_eq!(12, simple_hash(vec![3, 4, 1, 5], 5));
    }

    #[test]
    fn test_full_hash() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272",
                   full_hash("", 256));
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd",
                   full_hash("AoC 2017", 256));
    }
}
