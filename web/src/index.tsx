import React, { useState, createContext } from 'react';
import ReactDOM from 'react-dom';
import { Menu } from 'antd';
import Playground from './page/Playground';
import rust_fn_init, {
  interpret_lox,
} from '../../rs-package/lox_wasm/pkg/lox_wasm';

import 'antd/dist/antd.css';
import './index.css';

const PLAYGROUND = 'Playground';
const GRAMMER = 'Grammer';

export const WasmFunContext = createContext<{
  interpret_lox: (code: string) => void;
}>({
  interpret_lox: (code: string) => {},
});

function App() {
  const [active, setActive] = useState<string>(PLAYGROUND);
  
  return (
    <div className="App">
      <Menu
        mode="inline"
        theme="dark"
        selectedKeys={[active]}
        onClick={(item) => setActive(item.key)}
      >
        <Menu.Item key={PLAYGROUND}>{PLAYGROUND}</Menu.Item>
        <Menu.Item key={GRAMMER}>{GRAMMER}</Menu.Item>
      </Menu>
      <Playground show={active === PLAYGROUND} />
      <iframe
        style={{
          display: active === GRAMMER ? '' : 'none',
          flex: 'auto',
          height: '100%',
          border: 'none',
        }}
        title="grammer"
        src="https://craftinginterpreters.com/the-lox-language.html"
      />
    </div>
  );
}

(async () => {
  await rust_fn_init();

  ReactDOM.render(
    <React.StrictMode>
      <WasmFunContext.Provider
        value={{
          interpret_lox,
        }}
      >
        <App />
      </WasmFunContext.Provider>
    </React.StrictMode>,
    document.getElementById('root'),
  );
})();
