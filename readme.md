Emoji Encoder/Decoder
A Rust command-line tool for hiding and retrieving secret messages by encoding them into emoji sequences using Unicode variation selectors.

Overview
This project demonstrates a simple steganography technique where secret messages are hidden in plain sight by encoding them within emoji strings. Each message is encoded using a base emoji combined with variation selectors to represent bits of the message. An improved encoding scheme is used here to pack 4 bits (a nibble) per emoji pair by utilizing Unicode variation selectors U+FE00 through U+FE0F.

For example, the message "Hello" is encoded into a sequence of emoji pairs. When decoded, the program retrieves the original message.

Encoding Scheme
Encoding:
For each byte of the input message, split the byte into two 4-bit nibbles. Each nibble is then encoded as:

A base emoji (default is 😀, but this can be changed)
A variation selector from U+FE00 (representing 0) to U+FE0F (representing 15)
This means each byte becomes 2 emoji pairs (4 Unicode characters), making the overall encoding more efficient than one bit per emoji pair.

Decoding:
The program extracts the variation selectors from the emoji string, reassembles the nibbles into bytes, and converts them back to the original text.

Prerequisites
Rust – Install via rustup.
Installation
Clone the Repository (or create a new project):

bash
Copy
git clone https://github.com/yourusername/emoji_encoder.git
cd emoji_encoder
Or create a new Cargo project:

bash
Copy
cargo new emoji_encoder
cd emoji_encoder
Replace the Contents of src/main.rs:

Copy and paste the provided Rust code (the improved version) into src/main.rs.

Usage
Build the Project
Compile the project using Cargo:

bash
Copy
cargo build --release
Running the Program
The program supports two commands: encode and decode.

Encode a Message
To encode a message, use:

bash
Copy
cargo run -- encode "Your secret message" "😀"
Example:

bash
Copy
cargo run -- encode "Hello" "😀"
This will output an encoded string consisting of emoji and variation selectors.

Decode a Message
To decode an encoded string, use:

bash
Copy
cargo run -- decode "<your_encoded_string_here>"
Example:

Suppose you have the following encoded string:

Copy
😀︃😀︊😀︂😀︈
Run:

bash
Copy
cargo run -- decode "😀︃😀︊😀︂😀︈"
This should print the original secret message (e.g., :().

Example
Encoding "Hello":

bash
Copy
cargo run -- encode "Hello"
Output might be (using the improved nibble method):

Copy
😀︄😀︈😀︆😀︅😀︆😀︌😀︆😀︌😀︆😀︎
Decoding the above output:

bash
Copy
cargo run -- decode "😀︄😀︈😀︆😀︅😀︆😀︌😀︆😀︌😀︆😀︎"
The program should output:

nginx
Copy
Hello
Improvements
Efficiency:
The improved scheme encodes 4 bits per emoji pair by using the full range of variation selectors (U+FE00 to U+FE0F). This reduces the character overhead compared to encoding one bit per pair.

Customization:
You can easily change the base emoji by providing a different emoji as the third command-line argument when encoding.

Error Handling:
The program checks for incomplete sequences or invalid variation selectors during decoding and reports errors accordingly.

Contributing
Feel free to fork this repository and submit pull requests with improvements or bug fixes. Contributions that enhance the efficiency or functionality of the encoding/decoding process are welcome.

License
This project is licensed under the MIT License.

This README should provide a solid starting point for users to build, run, and understand the project. Enjoy encoding your secret messages!