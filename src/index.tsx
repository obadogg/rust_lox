import React, { useState } from 'react';
import ReactDOM from 'react-dom';
import { Menu } from 'antd';
import Playground from './web/Playground';
import * as rust_fn from '../rs-package/lox_wasm/pkg/lox_wasm';

import './index.css';

console.log(rust_fn, 'asdasd');

const PLAYGROUND = 'Playground';
const GRAMMER = 'Grammer';

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

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root'),
);
