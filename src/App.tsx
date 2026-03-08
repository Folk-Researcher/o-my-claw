import { useState } from 'react'
import './App.css'

// 导入Tauri API
import { invoke } from '@tauri-apps/api/core'

function App() {
  const [count, setCount] = useState(0)
  const [name, setName] = useState('World')
  const [greeting, setGreeting] = useState('')
  const [isLoading, setIsLoading] = useState(false)

  // 调用后端greet命令
  const handleGreet = async () => {
    setIsLoading(true)
    try {
      const response = await invoke('greet', { request: { name: name } })
      setGreeting(response.message)
    } catch (error) {
      console.error('Error calling greet:', error)
      setGreeting('Error: ' + error)
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <>
      <h1>o my claw</h1>
      
      {/* 计数器组件 */}
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      
      {/* 前后端通信测试 */}
      <div className="card">
        <h2>前后端通信测试</h2>
        <div className="input-group">
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="Enter your name"
            className="input"
          />
          <button 
            onClick={handleGreet}
            disabled={isLoading}
            className="button"
          >
            {isLoading ? 'Loading...' : 'Greet'}
          </button>
        </div>
        {greeting && (
          <div className="greeting">
            <h3>Response:</h3>
            <p>{greeting}</p>
          </div>
        )}
      </div>
    </>
  )
}

export default App