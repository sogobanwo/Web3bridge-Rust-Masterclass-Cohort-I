### Task-stylus-js

**GitHub:** [https://github.com/gboigwe/stylus-as-example](https://github.com/gboigwe/stylus-as-example)

**Description:**
Refactored the Stylus AssemblyScript prime calculator to solve the "minimum even prime" problem. Since 2 is mathematically the only even prime number, this implementation showcases how understanding the problem domain can lead to dramatic code simplification.


### Task: zig-stylus-assignment

**GitHub:** [https://github.com/gboigwe/zig-stylus](https://github.com/gboigwe/zig-stylus)

**Summary:**  
Successfully forked the zig-stylus repo, documented all files, and built WASM contracts. Contract structure passes Stylus validation, but discovered that the SDK's VM hook function names (like `storage_store_bytes32`) don't match the current Stylus runtime's available imports. This indicates the Zig SDK needs updates for compatibility with the latest Stylus version.

**Progress:** ✅ WASM compilation, ✅ Entrypoint recognition, ⚠️ VM hooks compatibility