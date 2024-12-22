# Age of Empires 2 Record parser

This is a WASM library to parse Age of Empires 2 DE save games

## Usage
To use the library, pass a ArrayBuffer to the `parse_rec` function. Example:


```html
<input type="file" id="fileElem">
```

```js
import wasm from "aoe2rec-js";

const fileElem = document.getElementById("fileElem");
fileElem.addEventListener("change", event => {
  const files = (event.target as HTMLInputElement).files
  if (!files) {
    replayFile.value = null
    return
  }
  const file = files[0]
  const reader = new FileReader();
  reader.onload = (event) => {
    wasm().then((instance) => {
      const rec = instance.parse_rec(event.target.result);
    });
  };
  reader.readAsArrayBuffer(file);

}, false);
```
