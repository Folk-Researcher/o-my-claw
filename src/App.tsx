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
  return (
    <div className="home-page">
      <h1>O My Claw</h1>
      <p>Welcome to O My Claw - OpenClaw Desktop Manager</p>

      <div className="feature-list">
        <h3>Features:</h3>
        <ul>
          <li>✅ Version Management - Download and manage OpenClaw versions</li>
          <li>⏳ Launch Control - Start/Stop OpenClaw instances</li>
          <li>⏳ Configuration - Manage OpenClaw settings</li>
          <li>⏳ Logs - View OpenClaw logs</li>
        </ul>
      </div>

      <div className="quick-start">
        <h3>Quick Start:</h3>
        <p>Click "Version Manager" in the navigation to get started.</p>
      </div>
    </div>
  )
}

export default App
