fn main() {
    let answer = day1::find_pair_with_sum("entries.txt", 2020);

    println!(
        "The two numbers that sum to 2020 are {} and {} and their product is {}",
        answer.0,
        answer.1,
        answer.0 * answer.1
    );

    let answer = day1::find_triplet_with_sum("entries.txt", 2020);

    println!(
        "The three numbers that sum to 2020 are {}, {}, and {} and their product is {}",
        answer.0,
        answer.1,
        answer.2,
        answer.0 * answer.1 * answer.2
    );
}
