/* eslint-disable react-hooks/exhaustive-deps */
import React, {
  useState,
  useRef,
  useCallback,
  useEffect,
  useContext,
} from 'react';
import Editor from '@monaco-editor/react';
import { Tabs, List, Button } from 'antd';
import { CloseCircleOutlined, CaretRightOutlined } from '@ant-design/icons';
import { debounce } from 'lodash';
import { WasmFunContext } from '../../index';
import DEFAULT_CODE from './defaultCode';

import './style.css';

const PROBLEMS = 'Problems';
const CONSOLE = 'Console';
const TAB_CONFIGS = [
  {
    key: CONSOLE,
    label: CONSOLE,
  },
  {
    key: PROBLEMS,
    label: PROBLEMS,
  },
];

function Playground({ show }: { show: boolean }) {
  const [activeKey, setActiveKey] = useState<string>(TAB_CONFIGS[0].key);
  const [errors, setErrors] = useState<string[]>([]);
  const [output, setOutput] = useState<Array<string | number>>([]);
  const wasmFun = useContext(WasmFunContext);
  const [code, setCode] = useState<string>(DEFAULT_CODE);
  const [loading, setLoading] = useState<boolean>(false);

  const handleRun = () => {
    setLoading(true);
    wasmFun.interpret_lox(code);
    setLoading(false);
  };

  useEffect(() => {
    const originalLog = console.log;
    const originalError = console.error;

    console.log = (...data: any[]) => {
      setOutput((pre) => [
        ...pre,
        ...data.map((item) => {
          if (item === null) {
            return 'null';
          }

          if (!(item instanceof Object)) {
            return String(item);
          }

          try {
            return JSON.stringify(item);
          } catch (err) {
            return Object.prototype.toString.apply(item);
          }
        }),
      ]);
      originalLog(...data);
    };

    console.error = (...data: any[]) => {
      let [msg] = data;
      msg = `${
        /\*\*\*\*\*\*[\s\S]*\*\*\*\*\*\*/i.exec(msg)?.[0] ?? 'Unknown error'
      }`;
      setErrors(msg.split('\n'));
      setActiveKey(PROBLEMS);
      originalError(...data);
    };
  }, []);

  return (
    <div className="playground" style={{ display: show ? '' : 'none' }}>
      <Editor
        height="65%"
        defaultLanguage="plaintext"
        defaultValue={DEFAULT_CODE}
        value={code}
        onChange={(value) => {
          setCode(value || '');
        }}
      />
      <div className="terminal">
        <Tabs
          activeKey={activeKey}
          onChange={setActiveKey}
          tabBarExtraContent={
            <>
              <Button
                loading={loading}
                icon={<CloseCircleOutlined />}
                style={{ marginRight: 16 }}
                onClick={() => setOutput([])}
              >
                Clear
              </Button>
              <Button
                type="primary"
                loading={loading}
                icon={<CaretRightOutlined />}
                disabled={errors.length > 0}
                onClick={handleRun}
              >
                Run
              </Button>
            </>
          }
        >
          {TAB_CONFIGS.map((item) => {
            let Content: React.ReactNode = null;
            if (item.key === PROBLEMS) {
              Content = (
                <List
                  header={null}
                  footer={null}
                  bordered
                  dataSource={errors}
                  renderItem={(item) => <List.Item>{item}</List.Item>}
                />
              );
            } else if (item.key === CONSOLE) {
              Content = (
                <List
                  header={null}
                  footer={null}
                  bordered
                  dataSource={output}
                  renderItem={(item) => <List.Item>{item}</List.Item>}
                />
              );
            }

            let tabName = item.key;
            if (item.key === PROBLEMS && errors.length > 0) {
              tabName += `(${errors.length - 3})`;
            }

            return (
              <Tabs.TabPane tab={tabName} key={item.key}>
                {Content}
              </Tabs.TabPane>
            );
          })}
        </Tabs>
      </div>
    </div>
  );
}

export default Playground;
