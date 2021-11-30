import React, { useState } from 'react';
import ReactDOM from 'react-dom';
import { Menu } from 'antd';
import Playground from './page/Playground';
import rust_fn_init,{ interpret_lox } from '../../rs-package/lox_wasm/pkg/lox_wasm';

import 'antd/dist/antd.css';
import './index.css';

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
 
(async () => {
  let rust_module = await rust_fn_init();

  console.log(interpret_lox(`
    var sum = 1;
    for(var i = 0;i < 10000000; i = i + 1){
        sum = sum + 1;
    }
    print sum;
  `),'asdasd')

  ReactDOM.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
    document.getElementById('root'),
  );
})()


