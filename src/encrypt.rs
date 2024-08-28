fn pad_string(string: &str, chunk_size: usize) -> String {
    let target_len = if string.len() % chunk_size != 0 {
        ((string.len() / chunk_size) + 1) * chunk_size
    } else {
        string.len()
    };
    let mut padded = string.to_string();

    if string.len() < target_len {
        let pad_len = target_len - string.len();
        padded.push_str(&" ".repeat(pad_len));
    }

    return padded;
}

pub fn split_to_chunks(input: &str, chunk_size: usize) -> Vec<String> {
    let padded = pad_string(input, chunk_size);
    let iterations = padded.len() / chunk_size;

    let mut output: Vec<String> = Vec::new();

    for i in 0..iterations {
        let start = i * chunk_size;
        let end = (i + 1) * chunk_size;

        let chunk = &padded[start..end];
        println!("Pushing {}", i);
        output.push(chunk.to_string());
    }

    return output;
}
