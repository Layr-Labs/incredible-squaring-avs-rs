# Incredible Squaring Avs 



To run 

- Start anvil in a separate terminal 
```
make start-anvil
```
- Then run the following command to start the avs

```bash
cargo run --bin incredible-squaring-avs  start --chain-id 31337  --ecdsa-keystore-path ./crates/testing-utils/src/ecdsakeystore.json --ecdsa-keystore-password test  --bls-keystore-path ./crates/testing-utils/src/blskeystore.json --bls-keystore-password testpassword  --operator-id 0xb345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3  --operator-address 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266 --register-operator --operator-to-avs-registration-sig-salt b345f720903a3ecfd59f3de456dd9d266c2ce540b05e8c909106962684d9afa3 --socket hello --quorum-number 00
```


