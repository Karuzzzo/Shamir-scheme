# Shamir scheme
This is an algorithm for implementing key separation using the Shamir scheme. 

### Installation and launch
To launch program, you need to download folder, cd to it, and run with one of possible parameters:


 - Split mode:
 
```sh
$ cargo run main.rs split
```

After selecting a split mode, you will be asked to input key (in HEX format) to split. You can type mine, from the tests section, or type your own.  

Then, type number of public keys you need, and number of keys required to restore secret.

 - Recover mode:
```sh
$ cargo run main.rs recover
```

After selecting a recover mode, you will be asked to input keys, generated in split mode. You must type them, separated with enter key. After you typed all of them, press enter one more time. 

>Type exact number of keys, required to restore 
>our polynomial.
>In other case, Lagrange interpolation breaks for some reason. 
>That's not my fault, Lagrange algorithm can only work with exactly k points,
>where k - 1 is degree of polynomial.

### Tests

### Warning!
My code is not optimised, and doesnt handle most of the errors(for its simplicity). So try not to make any mistakes in input.

For split test, used key with length of 64 symbols:
044f043b044e0431043b044e043f04380442044c043f04380432043e04450434

You can use any number, but not bigger than field module P.

As field module number P, i used simple number:
fffffffff44e0431043b044e043f04380442044c043f04380432043e044505fb

[Generator of big simple numbers](https://ru.numberempire.com/primenumbers.php)

output for input key listed above, total 20 random keys
for restoring back our secret, use any 3 keys:
1k4a6c1bfdec25186df2ed8efcb3f20d101511d13452d1879ad9e8ef0555b57328
2k04e26ecabc23f99834f743eee1460fed5100714c94babb06daabec828de30b23
3k33b1fca16898abe0ce932772907a1107bc4fe8e0ce39a2b40aad00f3b112d220
4kd6dac581f1832f47bfc13987c18e105f570037f0ff4e3ea269ec2c58bf44c81f
5kee5cc96c62957f9c044675e0704309bc1ccf5a3123b98a99f4376a73b433e725
6k7a380860bbcf9cdd9c22dc7c9c98fd1e0dbd4fa13b7b869aa98ebb448fe02f32
7k7a6c825ef17f8b3d8b9171aa4aceeebd2e0c1c8d4ad336dc8e242309568ea641
8keefa376703a54abbd29235697ae4de997dbbc0f551c09b5fa1f7a1c2083f4c52
9kd7e12778fdf2d7276cea236c289bc87af88a388d4c04afebe0d73330a0ad1b6a
10k35215294e06830805a993bb253f3ac619e778355399f74814ac2d7551fd81389
11k06bab8ba9f535af79fda828a012b8e8573c5a5991ecfed57e3ec926d8a053aaa
12k4cad59ea3ab4568d3cadf7f330436ee678749f58fb961a6fac546479df3490cd
13k06f93623be3d1f102cd8979fdcfc494ca8426c48cbb2f7909fc8493c1b210ff7
14k359e4d671e3bb8b1749565de0b9521f0077110b4936588f2c27a44f2420fbe23
15kd89c9fb45ab0237113e462adbc0df8d096008c9c52adce96146a579c54009b51
16keff42d0b7f4c5b1e068a89c0ea27c9b64faedbb4054cc44291667cfc4caea186
17k7ba4f56c8c105fb84c87db1795e294a1347bfdfbab4269f8396eb5122c19d0c2
18k7baef8d7754a3570ea175affc37d5dc948a9f7bf48cdc3ef10b5041bf6872f00
19kf012374c3af9dc47df39097972f8252e8c38c8feddeed22717396a19abf6bc40
20kd8ceb0cae8d1500c27b1e236a013e698fae66d6e6666906848c9e2cd48237287

Note, that the number before char 'k' represents number of key, and also X coordinate of point
