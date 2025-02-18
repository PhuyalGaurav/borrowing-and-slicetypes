fn main() {
    let word: &str = "Gaurav Phuyal";
    let word_length: usize = calc_len(word);
    println!("The length of {word} is {word_length}");

    mutable_refrence();
    dangaling_refrence();

    let first_word_len: usize = first_word_no_slice(word);
    println!("The length of the first word is : {first_word_len}");

    string_slices();

    let first_word: &str = first_word_with_slice(&word);
    println!("The first word is: {first_word}");

    array_slicing();
}

fn calc_len(a_word: &str) -> usize {
    let mut len: usize = 0;
    for i in a_word.as_bytes().iter() {
        len += 1;
        println!("Iterating letter : {i}")
    }
    return len;
}

fn mutable_refrence() {
    let mut s: String = String::from("Gaurav");

    // this will work because there is not refrence to s which is mutable yet.
    let s_ref: &String = &s;
    println!("A refrence of s : {s_ref}");

    let s_ref_mut: &mut String = &mut s;

    // Two mutable refrence cant be created within the same scope, So this wont work.
    // let s_ref_mut_2: &mut String = &mut s;

    // An immutable refrence cannot be created when a mutable refrence is still in socpe. So this also wont work.
    // let s_ref: &String = &s;

    add_is_good(s_ref_mut);
    fn add_is_good(some_string: &mut String) {
        some_string.push_str(", Is good.")
    }

    println!("Mutated s : {s_ref_mut}")
}

fn dangaling_refrence() {
    // ref_dangle = dangle(s)

    // This wont work because s will be dropped when it goes out of scope and the return value of dangle() will be refrencing nothing.
    // fn dangle() -> &String {
    //   let s: String = String::from("Gaurav");
    //
    //   return &s;
    //}

    // So it will only work if the string is returned instead of a refrence

    let new_s: String = no_dangle();

    fn no_dangle() -> String {
        let s: String = String::from("Gaurav");
        s
    }

    println!("No dangle : {new_s}")
}

fn first_word_no_slice(word: &str) -> usize {
    for (i, &item) in word.as_bytes().iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return word.len();
}

fn string_slices() {
    let word: String = String::from("Gaurav Phuyal");
    // Here gaurav contains a refrence of the word upto the 6th character which will be "Gaurav"
    let gaurav: &str = &word[..6];

    println!("{gaurav}");
}

fn first_word_with_slice(word: &str) -> &str {
    for (i, &item) in word.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &word[..i];
        }
    }
    return &word;
}

fn array_slicing() {
    let a: [usize; 5] = [1, 2, 3, 4, 5];

    // like sting the array is also sliced from the second element upto the penultimate element.
    let sliced_array: &[usize] = &a[1..a.len() - 1];
    for (i, i_num) in sliced_array.iter().enumerate() {
        println!("element no. {i_num} {i}");
    }
}
