# gtp-rs
A Guitar Pro File Format Parser(GTP), supports all gtp versions(gp3,gp4,gp5,gpx).


# Example On Rust


# Example On WASM

```javascript
//direct from url
const gtp = GTP.new_from_url('https://xxxxxxx/xxx/xxx/xx.gp4');
const data = gtp.parse();    

//from array buffer.
const gtp = GTP.new_from_array_buffer(data);
const data = gtp.parse(); 
```

# Road Maps

- [] basic implementation.
- [] confirm complicated cross versions.
- [] WASM supports.
