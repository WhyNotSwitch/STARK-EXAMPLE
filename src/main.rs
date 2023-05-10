
use std::time::Instant;

use examples::{{fibonacci, ExampleOptions, ExampleType}, StarkProof};
use ws_sdk::{log};

#[no_mangle]
pub extern "C" fn start(resource_id: i32) -> i32 {
    log::log_info(format!("func start called with resource id {resource_id}").as_str()).unwrap();

    let options = ExampleOptions::new(ExampleType::Fib { sequence_length: 64 });
    let example = fibonacci::fib2::get_example(&options, 64).expect("The example failed to initialize.");

    let now = Instant::now();
    let example = example.as_ref();
    let proof = example.prove();
    log::log_info(
        format!("---------------------\nProof generated in {} ms",
        now.elapsed().as_millis()).as_str()
    ).unwrap();

    let proof_bytes = proof.to_bytes();
    log::log_info(format!("Proof size: {:.1} KB", proof_bytes.len() as f64 / 1024f64).as_str()).unwrap();
    // let conjectured_security_level = options.get_proof_security_level(&proof, true);

    // #[cfg(feature = "std")]
    // {
    //     let proven_security_level = options.get_proof_security_level(&proof, false);
    //     debug!(
    //         "Proof security: {} bits ({} proven)",
    //         conjectured_security_level, proven_security_level,
    //     );
    // }

    // #[cfg(not(feature = "std"))]
    // debug!("Proof security: {} bits", conjectured_security_level);

    // #[cfg(feature = "std")]
    // debug!(
    //     "Proof hash: {}",
    //     hex::encode(blake3::hash(&proof_bytes).as_bytes())
    // );

    // verify the proof
    log::log_info("---------------------").unwrap();
    let parsed_proof = StarkProof::from_bytes(&proof_bytes).unwrap();
    assert_eq!(proof, parsed_proof);
    let now = Instant::now();
    match example.verify(proof) {
        Ok(_) =>
            log::log_info(format!(
                "Proof verified in {:.1} ms",
                now.elapsed().as_micros() as f64 / 1000f64
            ).as_str()).unwrap(),
        Err(msg) => log::log_error(format!("Failed to verify proof: {}", msg).as_str()).unwrap()
    }
    log::log_info("============================================================").unwrap();
    0
}

fn main() {
    println!("Hello, world!");
    start(23);
}
