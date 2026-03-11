import { useState } from 'react'
import './App.css'
import { VersionManager } from './pages'

function App() {
  const [currentPage, setCurrentPage] = useState<'home' | 'version'>('home')

  return (
    <div className="app">
      <nav className="app-nav">
        <button 
          className={`nav-button ${currentPage === 'home' ? 'active' : ''}`}
          onClick={() => setCurrentPage('home')}
        >
          Home
        </button>
        <button 
          className={`nav-button ${currentPage === 'version' ? 'active' : ''}`}
          onClick={() => setCurrentPage('version')}
        >
          Version Manager
        </button>
      </nav>

      <main className="app-main">
        {currentPage === 'home' ? (
          <HomePage />
        ) : (
          <VersionManager />
        )}
      </main>
    </div>
  )
}

// 首页组件
function HomePage() {
  const [count, setCount] = useState(0)

  return (
    <div className="home-page">
      <h1>O My Claw</h1>
      <p>Welcome to O My Claw - OpenClaw Desktop Manager</p>
      
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Click the button to test React state
        </p>
      </div>

      <div className="feature-list">
        <h3>Features:</h3>
        <ul>
          <li>✅ Version Management - Download and manage OpenClaw versions</li>
          <li>⏳ Launch Control - Start/Stop OpenClaw instances</li>
          <li>⏳ Configuration - Manage OpenClaw settings</li>
          <li>⏳ Logs - View OpenClaw logs</li>
        </ul>
      </div>
    </div>
  )
}

export default App
