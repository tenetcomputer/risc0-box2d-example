use methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover,
};

// mod hello_world;

fn main() {
    // Pick two numbers
    let a: u64 = 17;
    let b: u64 = 23;

    // hello_world::hello_world();

    // Multiply them inside the ZKP
    // First, we make the prover, loading the 'multiply' method
    let mut prover = Prover::new(MULTIPLY_ELF, MULTIPLY_ID).expect(
        "Prover should be constructed from valid method source code and corresponding method ID",
    );

    // Next we send a & b to the guest
    prover.add_input_u32_slice(&to_vec(&a).expect("should be serializable"));
    prover.add_input_u32_slice(&to_vec(&b).expect("should be serializable"));
    // Run prover & generate receipt

    println!("Proof started...");
    let receipt = prover
        .run()
        .expect("Should be able to prove valid code that fits in the cycle count.");
    println!("Proof finished!");

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = from_slice(&receipt.journal).expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    // Print an assertion
    println!("I know the factors of {}, and I can prove it!", c);

    // Here is where one would send 'receipt' over the network...

    // Verify receipt, panic if it's wrong
    receipt.verify(&MULTIPLY_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );
}
