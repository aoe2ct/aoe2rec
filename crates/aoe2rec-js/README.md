# Age of Empires 2 Record parser

This is a WASM library to parse Age of Empires 2 DE save games

## Usage
To use the library, pass a ArrayBuffer to the `parse_rec` function. Example:


```html
<input type="file" id="fileElem">
```

```js
import { parse_rec } from "aoe2rec-js";

const fileElem = document.getElementById("fileElem");
fileElem.addEventListener("change", event => {
  const files = (event.target as HTMLInputElement).files
  if (!files) {
    replayFile.value = null
    return
  }
  const file = files[0]
  const reader = new FileReader();
  reader.addEventListener('loadend', (event) => {
    try {
      const rec = parse_rec(event.target.result);
    } catch (error) {
      // It's important to catch errors because not all recs are guaranteed to parse correctly
      console.error("Failed to parse game");
    };
  }, false);
  reader.readAsArrayBuffer(file);

}, false);
```
