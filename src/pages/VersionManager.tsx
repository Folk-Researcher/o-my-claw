import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './VersionManager.css';

interface Release {
  tag_name: string;
  name: string;
  body: string;
  published_at: string;
  html_url: string;
}

export function VersionManager() {
  const [releases, setReleases] = useState<Release[]>([]);
  const [selectedRelease, setSelectedRelease] = useState<Release | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [actionLoading, setActionLoading] = useState(false);
  const [logs, setLogs] = useState<string[]>([]);
  const [versionStatus, setVersionStatus] = useState<string>('unknown');

  useEffect(() => {
    loadReleases();
  }, []);

  useEffect(() => {
    if (selectedRelease) {
      checkVersionStatus(selectedRelease.tag_name);
    }
  }, [selectedRelease]);

  const loadReleases = async () => {
    try {
      setLoading(true);
      const data = await invoke<Release[]>('list_releases');
      setReleases(data);
      if (data.length > 0) {
        setSelectedRelease(data[0]);
      }
    } catch (err) {
      setError('Failed to load releases: ' + err);
    } finally {
      setLoading(false);
    }
  };

  const checkVersionStatus = async (tag: string) => {
    try {
      const status = await invoke<string>('check_local_version', { tag });
      setVersionStatus(status);
    } catch (err) {
      setVersionStatus('unknown');
    }
  };

  const handleCheckout = async () => {
    if (!selectedRelease) return;
    
    setActionLoading(true);
    addLog(`Checking out version ${selectedRelease.tag_name}...`);
    
    try {
      await invoke('checkout_version', { tag: selectedRelease.tag_name });
      addLog('Checkout completed!');
      checkVersionStatus(selectedRelease.tag_name);
    } catch (err) {
      addLog('Error: ' + err);
    } finally {
      setActionLoading(false);
    }
  };

  const handleCompile = async () => {
    if (!selectedRelease) return;
    
    setActionLoading(true);
    addLog(`Compiling version ${selectedRelease.tag_name}...`);
    
    try {
      await invoke('compile_version', { tag: selectedRelease.tag_name });
      addLog('Compilation completed!');
      checkVersionStatus(selectedRelease.tag_name);
    } catch (err) {
      addLog('Error: ' + err);
    } finally {
      setActionLoading(false);
    }
  };

  const handleStart = async () => {
    if (!selectedRelease) return;
    
    setActionLoading(true);
    addLog(`Starting OpenClaw ${selectedRelease.tag_name}...`);
    
    try {
      await invoke('start_openclaw', { tag: selectedRelease.tag_name });
      addLog('OpenClaw started!');
    } catch (err) {
      addLog('Error: ' + err);
    } finally {
      setActionLoading(false);
    }
  };

  const handleStop = async () => {
    setActionLoading(true);
    addLog('Stopping OpenClaw...');
    
    try {
      await invoke('stop_openclaw');
      addLog('OpenClaw stopped!');
    } catch (err) {
      addLog('Error: ' + err);
    } finally {
      setActionLoading(false);
    }
  };

  const addLog = (message: string) => {
    setLogs(prev => [...prev, `[${new Date().toLocaleTimeString()}] ${message}`]);
  };

  const getStatusText = (status: string) => {
    switch (status) {
      case 'not_cloned': return '🔴 Not Installed';
      case 'cloned': return '🟡 Source Downloaded';
      case 'compiled': return '🟢 Ready to Run';
      case 'running': return '✅ Running';
      default: return '❓ Unknown';
    }
  };

  if (loading) {
    return <div className="version-manager loading">Loading releases...</div>;
  }

  if (error) {
    return <div className="version-manager error">{error}</div>;
  }

  return (
    <div className="version-manager">
      <h1>OpenClaw Version Manager</h1>
      
      <div className="version-manager-content">
        {/* 左侧：版本列表 */}
        <div className="releases-sidebar">
          <h3>Available Versions</h3>
          <div className="releases-list">
            {releases.map((release) => (
              <div
                key={release.tag_name}
                className={`release-item ${selectedRelease?.tag_name === release.tag_name ? 'selected' : ''}`}
                onClick={() => setSelectedRelease(release)}
              >
                <div className="release-tag">{release.tag_name}</div>
                <div className="release-date">
                  {new Date(release.published_at).toLocaleDateString()}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* 右侧：版本详情 */}
        <div className="version-detail">
          {selectedRelease && (
            <>
              <div className="version-info">
                <h2>{selectedRelease.name}</h2>
                <div className="version-meta">
                  <span>Tag: {selectedRelease.tag_name}</span>
                  <span>Released: {new Date(selectedRelease.published_at).toLocaleDateString()}</span>
                </div>
                <div className="version-status">
                  Status: {getStatusText(versionStatus)}
                </div>
                <div className="version-description">
                  <pre>{selectedRelease.body}</pre>
                </div>
              </div>

              <div className="version-actions">
                <button 
                  onClick={handleCheckout} 
                  disabled={actionLoading}
                  className="button"
                >
                  {actionLoading ? 'Processing...' : 'Checkout Source'}
                </button>
                <button 
                  onClick={handleCompile} 
                  disabled={actionLoading}
                  className="button"
                >
                  {actionLoading ? 'Processing...' : 'Compile'}
                </button>
                <button 
                  onClick={handleStart} 
                  disabled={actionLoading}
                  className="button"
                >
                  {actionLoading ? 'Processing...' : 'Start'}
                </button>
                <button 
                  onClick={handleStop} 
                  disabled={actionLoading}
                  className="button"
                >
                  {actionLoading ? 'Processing...' : 'Stop'}
                </button>
              </div>

              <div className="process-console">
                <h3>Process Log</h3>
                <div className="console-output">
                  {logs.length === 0 ? (
                    <div className="console-placeholder">No logs yet...</div>
                  ) : (
                    logs.map((log, index) => (
                      <div key={index} className="console-line">{log}</div>
                    ))
                  )}
                </div>
                <button onClick={() => setLogs([])} className="button clear-button">
                  Clear Logs
                </button>
              </div>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
