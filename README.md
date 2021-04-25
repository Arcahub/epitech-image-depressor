# Epitech - ImageDepressor

This is a little tool written in rust for the epitech haskell projet "Image Compressor". In the projet the student must read a list of pixels from a text file and use the K-Means algorithm to output them in different clusters. As this is said even though the projet name is "Image **_Compressor_**" there isn't any image involved. So I writted this tool to either get a list of pixel from a real image or create an image from the expected output of the project.

## How to install

You need to have rust installed and then first clone this repo:

```
git clone https://github.com/Arcahub/epitech-image-depressor.git
```

and then build the binary

```
cargo build --release && cp target/release/imagedepressor .
```

## How to use

To run the binary use `-i` to set input file and `-o` to set output file. The conversion while be done automatically depending on the input file.

```
./imagedepressor --input_path <input_path> --output_path <output_path>
```

If the input file is an img it while output a list of pixel like this:

```
(0,1) (98,99,233)
(2,0) (88,77,211)
(0,2) (45,12,167)
(1,0) (33,16,94)
(1,1) (78,8,9)
(1,2) (20,27,67)
(2,1) (1,56,37)
(0,0) (66,20,26)
(2,2) (15,89,40)
```

Or else it will try to read the K-Means clusters in the expected format and output an image. The exepcted input must be formatted like this:

```
--
(8,72,38)
-
(2,1) (1,56,37)
(2,2) (15,89,40)
--
(48,16,72)
-
(0,2) (45,12,167)
(1,0) (33,16,94)
(1,1) (78,8,9)
(1,2) (20,27,67)
(0,0) (66,20,26)
--
(93,88,222)
-
(0,1) (98,99,233)
(2,0) (88,77,211)
```
