use arrayvec::ArrayVec;

pub fn run(input: &[u8]) -> i64 {
    with_vec_drain(input)
}

pub fn with_arrayvec(input: &[u8]) -> i64 {
    let mut stack = ArrayVec::<_, 20>::new();
    let mut corrupted = false;
    let mut scores = ArrayVec::<_, 100>::new();

    #[inline(always)]
    fn push_score(stack: &mut ArrayVec<u8, 20>, scores: &mut ArrayVec<i64, 100>) {
        let score = stack.drain(..).rev().fold(0, |score, byte| {
            score * 5
                + match byte {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    _ => 4,
                }
        });

        scores.push(score);
    }

    for &byte in input {
        match byte {
            b'(' | b'[' | b'{' | b'<' => stack.push(byte),
            b')' => corrupted |= stack.pop() != Some(b'('),
            b']' => corrupted |= stack.pop() != Some(b'['),
            b'}' => corrupted |= stack.pop() != Some(b'{'),
            b'>' => corrupted |= stack.pop() != Some(b'<'),
            _ => {
                if !corrupted {
                    push_score(&mut stack, &mut scores);
                } else {
                    corrupted = false;
                    stack.clear();
                }
            }
        }
    }

    if input.last() != Some(&b'\n') && !corrupted {
        push_score(&mut stack, &mut scores);
    }

    scores.sort_unstable();

    scores[scores.len() / 2]
}

pub fn with_vec_drain(input: &[u8]) -> i64 {
    let mut stack = Vec::with_capacity(20);
    let mut corrupted = false;
    let mut scores = Vec::with_capacity(100);

    #[inline(always)]
    fn push_score(stack: &mut Vec<u8>, scores: &mut Vec<i64>) {
        let score = stack.drain(..).rev().fold(0, |score, byte| {
            score * 5
                + match byte {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    _ => 4,
                }
        });

        scores.push(score);
    }

    for &byte in input {
        match byte {
            b'(' | b'[' | b'{' | b'<' => stack.push(byte),
            b')' => corrupted |= stack.pop() != Some(b'('),
            b']' => corrupted |= stack.pop() != Some(b'['),
            b'}' => corrupted |= stack.pop() != Some(b'{'),
            b'>' => corrupted |= stack.pop() != Some(b'<'),
            _ => {
                if !corrupted {
                    push_score(&mut stack, &mut scores);
                } else {
                    corrupted = false;
                    stack.clear();
                }
            }
        }
    }

    if input.last() != Some(&b'\n') && !corrupted {
        push_score(&mut stack, &mut scores);
    }

    scores.sort_unstable();

    scores[scores.len() / 2]
}

pub fn with_vec_clear(input: &[u8]) -> i64 {
    let mut stack = Vec::with_capacity(20);
    let mut corrupted = false;
    let mut scores = Vec::with_capacity(100);

    #[inline(always)]
    fn push_score(stack: &Vec<u8>, scores: &mut Vec<i64>) {
        let score = stack.iter().rev().fold(0, |score, byte| {
            score * 5
                + match byte {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    _ => 4,
                }
        });

        scores.push(score);
    }

    for &byte in input {
        match byte {
            b'(' | b'[' | b'{' | b'<' => stack.push(byte),
            b')' => corrupted |= stack.pop() != Some(b'('),
            b']' => corrupted |= stack.pop() != Some(b'['),
            b'}' => corrupted |= stack.pop() != Some(b'{'),
            b'>' => corrupted |= stack.pop() != Some(b'<'),
            _ => {
                if !corrupted {
                    push_score(&stack, &mut scores);
                } else {
                    corrupted = false;
                }

                stack.clear();
            }
        }
    }

    if input.last() != Some(&b'\n') && !corrupted {
        push_score(&mut stack, &mut scores);
    }

    scores.sort_unstable();

    scores[scores.len() / 2]
}
