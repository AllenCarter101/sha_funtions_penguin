use std::fs::File;
use std::io::Read;
use sha2::{Sha256, Digest};

/// .
///
/// # Panics
///
/// Panics if 

   
     fn sha256_file(file: &str) -> String {
        let mut f = File::open(file).expect("file not found");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("failed to read file");
        let mut hasher = Sha256::new();
        hasher.update(&buffer);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
   
     fn compare_and_verify(file: &str, expected_sum: &str) -> bool {
        let computed_sum = sha256_file(file);
            if computed_sum != expected_sum {
                panic!("File contents do not match");
            }
                else { 
                    computed_sum == expected_sum
                }
    }
   
    fn main (){
        let end_result = sha256_file("winblows");
    
    println!("{}", &end_result);
    let end_compare = compare_and_verify("winblows", &end_result);
    
    println!("{}", end_compare);
}
    
    
        #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_sha256_file() {
            let expected_sum = "ccf58c31bf7e29e4ba535562897475210e386806565e444898feb6359178a2c6";
            let result = sha256_file("winblows");
            assert_eq!(expected_sum, result);

            // handle failure
        }
   
        #[test]
        fn test_compare_and_verify() {
            let expected_sum = "ccf58c31bf7e29e4ba535562897475210e386806565e444898feb6359178a2c6";
            let result = compare_and_verify("winblows", expected_sum);
            assert!(result);   
        }
    }