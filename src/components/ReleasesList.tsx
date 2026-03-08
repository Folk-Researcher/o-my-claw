import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Release {
  tag_name: string;
  name: string;
  body: string;
  published_at: string;
  html_url: string;
}

interface ReleasesListProps {
  onSelectRelease: (release: Release) => void;
  selectedTag: string | null;
}

export function ReleasesList({ onSelectRelease, selectedTag }: ReleasesListProps) {
  const [releases, setReleases] = useState<Release[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadReleases();
  }, []);

  const loadReleases = async () => {
    try {
      setLoading(true);
      const data = await invoke<Release[]>('list_releases');
      setReleases(data);
    } catch (err) {
      setError('Failed to load releases: ' + err);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return <div className="releases-loading">Loading releases...</div>;
  }

  if (error) {
    return <div className="releases-error">{error}</div>;
  }

  return (
    <div className="releases-list">
      <h3>Available Versions</h3>
      <div className="releases-container">
        {releases.map((release) => (
          <div
            key={release.tag_name}
            className={`release-item ${selectedTag === release.tag_name ? 'selected' : ''}`}
            onClick={() => onSelectRelease(release)}
          >
            <div className="release-tag">{release.tag_name}</div>
            <div className="release-date">
              {new Date(release.published_at).toLocaleDateString()}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
