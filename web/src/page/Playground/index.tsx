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
import rust_fn_init, {
  interpret_lox,
} from '../../../../rs-package/lox_wasm/pkg/lox_wasm';

import './style.css';

const PROBLEMS = 'Problems';
const CONSOLE = 'Console';
const TAB_CONFIGS = [
  {
    key: PROBLEMS,
    label: PROBLEMS,
  },
  {
    key: CONSOLE,
    label: CONSOLE,
  },
];

const DEFAULT_CODE = `class Person {
  init(name, birth) {
    this.name = name;
    this.birth = birth;
  }

  introduceMySelf() {
    print "my name is " + this.name;
    print "my birth is " + this.birth";
    print "thanks for coming";
    return "介绍结束";
  }
}

var me = Person("aadonkeyz", "1995");
print me.introduceMySelf();
`;

function Playground({ show }: { show: boolean }) {
  const [activeKey, setActiveKey] = useState<string>(TAB_CONFIGS[0].key);
  const [errors, setErrors] = useState<string[]>([]);
  const [output, setOutput] = useState<Array<string | number>>([]);
  const wasmFun = useContext(WasmFunContext);
  const [code, setCode] = useState<string>(DEFAULT_CODE);

  // const compilerRef = useRef<Compiler>();

  // const handleCodeChange = useCallback(
  //   debounce((code: string) => {
  //     const compiler = new Compiler(code);
  //     compilerRef.current = compiler;
  //     compiler.analysis();
  //     // @ts-ignore
  //     window.zzz = compiler;
  //     const newErrors = compiler.scanner.errors
  //       .concat(compiler.parser.errors, compiler.scopeAnalyst.errors)
  //       .map((item) => {
  //         return `${item.message} in line ${item.line} column ${item.column}.`;
  //       });
  //     setErrors(newErrors);
  //   }, 500),
  //   [],
  // );

  const handleRun = () => {
    console.log(interpret_lox);
    // console.log(code);
    interpret_lox(code);
  };

  useEffect(() => {
    const originalLog = console.log;
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

    // handleCodeChange(DEFAULT_CODE);
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
                danger
                icon={<CloseCircleOutlined />}
                style={{ marginRight: 16 }}
                onClick={() => setOutput([])}
              >
                <span style={{ transform: 'translateY(-1px)' }}>Clear</span>
              </Button>
              <Button
                style={{ background: '#52c41a' }}
                icon={<CaretRightOutlined />}
                disabled={errors.length > 0}
                onClick={handleRun}
              >
                <span style={{ transform: 'translateY(-1px)' }}>Run</span>
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
              tabName += `(${errors.length})`;
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
