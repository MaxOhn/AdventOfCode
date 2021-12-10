use arrayvec::ArrayVec;

pub fn run(input: &[u8]) -> i64 {
    with_vec(input)
}

pub fn with_arrayvec(input: &[u8]) -> i64 {
    let mut stack = ArrayVec::<_, 20>::new();
    let mut answer = 0;

    for &byte in input {
        match byte {
            b'(' | b'[' | b'{' | b'<' => stack.push(byte),
            b')' => answer += (stack.pop() != Some(b'(')) as i64 * 3,
            b']' => answer += (stack.pop() != Some(b'[')) as i64 * 57,
            b'}' => answer += (stack.pop() != Some(b'{')) as i64 * 1197,
            b'>' => answer += (stack.pop() != Some(b'<')) as i64 * 25_137,
            _ => stack.clear(),
        }
    }

    answer
}

pub fn with_vec(input: &[u8]) -> i64 {
    let mut stack = Vec::with_capacity(20);
    let mut answer = 0;

    for &byte in input {
        match byte {
            b'(' | b'[' | b'{' | b'<' => stack.push(byte),
            b')' => answer += (stack.pop() != Some(b'(')) as i64 * 3,
            b']' => answer += (stack.pop() != Some(b'[')) as i64 * 57,
            b'}' => answer += (stack.pop() != Some(b'{')) as i64 * 1197,
            b'>' => answer += (stack.pop() != Some(b'<')) as i64 * 25_137,
            _ => stack.clear(),
        }
    }

    answer
}
