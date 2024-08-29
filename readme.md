# Usage

### Text files

`cargo run input.txt -o encrypted.txt -t encrypt` <br>
`cargo run encrypted.txt -o decrypted.txt -t decrypt`

### Images

`cargo run image.jpg -o encrypted_image.jpg -t encrypt` <br>
`cargo run encrypted_image.jpg -o decrypted_image.jpg -t decrypt`

(Should work for all files)

## How it works
The AES encryption is uses symmetric encryption, meaning the same shared key is use for both encryption and decryption of data. 

## Steps

### Encryption

* Read input file as bytes and store the original size of the file
* If the input is not a multiple of 16 - `block_size`, add padding to it
* Encrypt each block of `16 bytes`. Create a new vector of bytes with all encrypted blocks flattened. 
* Prepend the input with 4 bytes representing the original size of the file. 
* The output file then looks like this: original_size (4 bytes), encrypted input file + padding (the padding is also encrypted)
* Write the output vector to a file

### Decryption
* Read the input file as bytes.
* Store the first `4 bytes` representing the original size of the file and remove it from the input. 
* Decrypt each `16 byte` block and add them to the result vector of bytes. 
* Based on the original size, truncate the output vector - effectively removing the `padding` added in the encryption process.
* Write the output vector to a file