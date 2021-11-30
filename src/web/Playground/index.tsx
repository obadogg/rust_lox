/* eslint-disable react-hooks/exhaustive-deps */
import React, { useState, useRef, useCallback, useEffect } from 'react';
import Editor from '@monaco-editor/react';
import { Tabs, List, Button } from 'antd';
import { CloseCircleOutlined, CaretRightOutlined } from '@ant-design/icons';
import { debounce } from 'lodash';

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
    this.age = Date().getFullYear() - this.birth;
  }

  introduceMySelf() {
    print "my name is " + this.name;
    print "i am " + String(this.age) + " years old";
    print "thanks for coming";
  }
}

var me = Person("aadonkeyz", 1995);
me.introduceMySelf();
`;

function Playground({ show }: { show: boolean }) {
  const [activeKey, setActiveKey] = useState<string>(TAB_CONFIGS[0].key);
  const [errors, setErrors] = useState<string[]>([]);
  const [output, setOutput] = useState<Array<string | number>>([]);

  return (
    <div className="playground" style={{ display: show ? '' : 'none' }}>
      <Editor
        height="65%"
        defaultLanguage="plaintext"
        defaultValue={DEFAULT_CODE}
        onChange={(value) => {}}
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
