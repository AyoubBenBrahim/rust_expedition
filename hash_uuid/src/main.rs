
use uuid::Uuid;
use ring::digest;

fn hash_uuid(uuid: Uuid) -> Vec<u8> {
    let uuid_bytes = uuid.as_bytes();
    let digest = digest::digest(&digest::SHA256, uuid_bytes);
    digest.as_ref().to_vec()
}

fn verify_uuid(uuid: Uuid, hashed_uuid: &[u8]) -> bool {
    let uuid_bytes = uuid.as_bytes();
    let digest = digest::digest(&digest::SHA256, uuid_bytes);
    digest.as_ref() == hashed_uuid
}


fn hash_uuid_string(uuid_string: &str) -> String {
    let digest = digest::digest(&digest::SHA256, uuid_string.as_bytes());
    //digest.as_ref().to_vec()
    hex::encode(digest.as_ref())
}

fn verify_uuid_string(uuid_string: &str, hashed_uuid: &str) -> bool {
    let expected_hash = hash_uuid_string(uuid_string);
    hashed_uuid == expected_hash
}

/*
fn verify_uuid_string(uuid_string: &str, hashed_uuid: &[u8]) -> bool {
    let digest = digest::digest(&digest::SHA256, uuid_string.as_bytes());
    digest.as_ref() == hashed_uuid
}
*/

fn main() {
    // Generate a UUID
    let original_uuid = Uuid::new_v4().to_string();
    
    let fake_uuid = "550e8400-e29b-41d4-a716-446655440000";

    // Hash the UUID string
    let hashed_uuid = hash_uuid_string(&original_uuid);
    
    // Print the hashed UUID
    println!("Hashed UUID: {:?}", hashed_uuid);
    
    // Verify the hash
    let mut is_valid = verify_uuid_string(&original_uuid, &hashed_uuid);
    
    // Print the verification result
    if is_valid {
        println!("UUID hash verification successful");
    } else {
        println!("UUID hash verification failed");
    }


    is_valid = verify_uuid_string(&fake_uuid, &hashed_uuid);
    
    // Print the verification result
    if is_valid {
        println!("UUID hash verification successful");
    } else {
        println!("UUID hash verification failed");
    }


}
