# golden-circle-packing
generates Fibonacci sequence circles using image processing

> **WARNING** Computer Intensive | Work in Progress

#### Output:
```
[circle-position-x, circle-position-y, circle-radius, [r,g,b,a]]
```
```
[[299,296,8.0,[53,52,27,255]],[299,267,5.0,[74,73,34,255]],[299,281,3.0,[91,85,44,255]]]
```

RAW Image             | Generated Circles
:-------------------------:|:-------------------------:
<img src="https://github.com/JacobZwang/golden-circle-packing/blob/main/examples/portrait_2.png?raw=true" width="330" height="300" /> | <img src="https://github.com/JacobZwang/golden-circle-packing/blob/main/examples/portrait_2_circles.png?raw=true" width="320" height="300"/>
<img src="https://github.com/JacobZwang/golden-circle-packing/blob/main/examples/portrait.png?raw=true" width="300" height="300"/> |  <img src="https://github.com/JacobZwang/golden-circle-packing/blob/main/examples/portrait_circles.png?raw=true" width="330" height="300" />
<img src="https://github.com/JacobZwang/golden-circle-packing/blob/main/examples/lambo.jpg?raw=true" width="350" height="260" /> | <img src="https://github.com/JacobZwang/golden-circle-packing/blob/main/examples/lambo_circles.png?raw=true" width="350" height="260"/>
<img src="https://github.com/JacobZwang/golden-circle-packing/blob/main/examples/dog.jpg?raw=true" width="300" height="300"/> |  <img src="https://github.com/JacobZwang/golden-circle-packing/blob/main/examples/dog_circles.png?raw=true" width="330" height="300" />

### To Run
> Install Rust
> Install VS Code / Live Server Extension
> install git
```
git clone http://github.com/jacobzwang/golden-cricle-packing.git
```
```
cd golden-circle-packing
```
> run live server in root project dir
```
cargo run --release
```
> http://localhost:5500

> circles will appear as the are generated
