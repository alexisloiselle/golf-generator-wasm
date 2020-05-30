import React from "react";
import Main from "./components/Main";
import { useWasm, WASM_READY_STATE } from "./useWasm";

const App = () => {
  const [WasmProvider, wasmObject] = useWasm();

  return (
    <WasmProvider value={wasmObject}>
      <div id="App">
        {wasmObject.readyState === WASM_READY_STATE.READY && <Main />}
      </div>
    </WasmProvider>
  );
};

export default App;
