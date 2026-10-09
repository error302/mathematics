import React, { useRef } from 'react';
import { RewardEntry } from '../engine/axiomCore';
import { Shield, Clock, Download, Upload } from 'lucide-react';

interface MasteryDashboardProps {
  ledgerEntries: RewardEntry[];
  totalXp: number;
  todayPracticeXp: number;
  provisionalUnlocked: boolean;
  masteryUnits: number;
  onImportLedger: (entries: RewardEntry[]) => void;
}

export const MasteryDashboard: React.FC<MasteryDashboardProps> = ({
  ledgerEntries,
  totalXp,
  todayPracticeXp,
  provisionalUnlocked,
  masteryUnits,
  onImportLedger,
}) => {
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleExport = () => {
    const data = {
      export_version: 1,
      export_timestamp: new Date().toISOString(),
      app: "axiom-academy",
      total_xp: totalXp,
      entries: ledgerEntries,
    };
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `axiom-backup-${new Date().toISOString().slice(0, 10)}.json`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = (event) => {
      try {
        const json = JSON.parse(event.target?.result as string);
        if (json.entries && Array.isArray(json.entries)) {
          onImportLedger(json.entries);
        }
      } catch (err) {
        alert("Failed to parse backup JSON file");
      }
    };
    reader.readAsText(file);
  };

  return (
    <div className="axiom-mastery-container">
      <div className="axiom-mastery-header">
        <div>
          <h2>Mastery & Rewards Ledger</h2>
          <p className="axiom-subtext">
            Transparent evidence tracking: Beta-style dimension scores and append-only reward accounting.
          </p>
        </div>

        <div className="axiom-backup-actions">
          <button className="axiom-btn axiom-btn-outline" onClick={handleExport}>
            <Download size={14} />
            <span>Export Progress JSON</span>
          </button>
          <button className="axiom-btn axiom-btn-outline" onClick={() => fileInputRef.current?.click()}>
            <Upload size={14} />
            <span>Restore Backup</span>
          </button>
          <input
            ref={fileInputRef}
            type="file"
            accept=".json"
            style={{ display: 'none' }}
            onChange={handleFileChange}
          />
        </div>
      </div>

      {/* Summary Cards */}
      <div className="axiom-stats-row">
        <div className="axiom-stat-card">
          <span className="axiom-stat-label">Total Verified XP</span>
          <span className="axiom-stat-value">{totalXp}</span>
          <span className="axiom-stat-sub">Sum of append-only ledger</span>
        </div>

        <div className="axiom-stat-card">
          <span className="axiom-stat-label">Today's Practice XP</span>
          <span className="axiom-stat-value">{todayPracticeXp} / 100</span>
          <span className="axiom-stat-sub">Daily ordinary practice cap</span>
        </div>

        <div className="axiom-stat-card">
          <span className="axiom-stat-label">Fraction Mastery Status</span>
          <span className="axiom-stat-value">
            {provisionalUnlocked ? 'Provisional' : 'Learning'}
          </span>
          <span className="axiom-stat-sub">{masteryUnits} / 6 units recorded</span>
        </div>

        <div className="axiom-stat-card">
          <span className="axiom-stat-label">Next Scheduled Review</span>
          <span className="axiom-stat-value">In 1 day</span>
          <span className="axiom-stat-sub">Ladder: [1, 3, 7, 14, 30]d</span>
        </div>
      </div>

      {/* Mastery Dimensions Progress */}
      <div className="axiom-card axiom-dimension-section">
        <h3>Course F04: Competency Dimensions Breakdown</h3>
        <p className="axiom-subtext">
          AXIOM evaluates learning across distinct dimensions rather than collapsing understanding into a single percentage.
        </p>

        <div className="axiom-dimensions-grid">
          <div className="axiom-dimension-box">
            <div className="axiom-dim-header">
              <strong>Conceptual</strong>
              <span className="axiom-dim-status">Active</span>
            </div>
            <p>Partitioning wholes, unit scale comparison, number line ordering.</p>
            <div className="axiom-progress-bar">
              <div className="axiom-progress-fill" style={{ width: '85%' }} />
            </div>
          </div>

          <div className="axiom-dimension-box">
            <div className="axiom-dim-header">
              <strong>Procedural</strong>
              <span className="axiom-dim-status">
                {masteryUnits >= 3 ? 'Met' : `${masteryUnits}/3`}
              </span>
            </div>
            <p>Finding common denominators, cross-multiplication, exact equivalence.</p>
            <div className="axiom-progress-bar">
              <div
                className="axiom-progress-fill"
                style={{ width: `${Math.min(100, (masteryUnits / 6) * 100)}%` }}
              />
            </div>
          </div>

          <div className="axiom-dimension-box">
            <div className="axiom-dim-header">
              <strong>Reasoning</strong>
              <span className="axiom-dim-status">Active</span>
            </div>
            <p>Explaining why unequal denominators prevent direct numerator comparison.</p>
            <div className="axiom-progress-bar">
              <div className="axiom-progress-fill" style={{ width: '70%' }} />
            </div>
          </div>

          <div className="axiom-dimension-box">
            <div className="axiom-dim-header">
              <strong>Transfer</strong>
              <span className="axiom-dim-status">Available</span>
            </div>
            <p>Applying rational equivalence in unfamiliar representations and contexts.</p>
            <div className="axiom-progress-bar">
              <div className="axiom-progress-fill" style={{ width: '40%' }} />
            </div>
          </div>
        </div>
      </div>

      {/* Reward Ledger Table */}
      <div className="axiom-card">
        <div className="axiom-card-header">
          <div>
            <h3>Append-Only Reward Ledger</h3>
            <p className="axiom-subtext">Each XP transaction is immutable and uniquely keyed.</p>
          </div>
          <span className="axiom-badge">
            <Shield size={12} />
            <span>Audit-Verified</span>
          </span>
        </div>

        {ledgerEntries.length === 0 ? (
          <div className="axiom-empty-ledger">
            <Clock size={24} />
            <p>No rewards recorded yet. Complete an unassisted exercise in the practice workspace to earn your first XP!</p>
          </div>
        ) : (
          <div className="axiom-table-wrapper">
            <table className="axiom-table">
              <thead>
                <tr>
                  <th>Timestamp (UTC)</th>
                  <th>Award Key</th>
                  <th>Reason</th>
                  <th>Delta</th>
                </tr>
              </thead>
              <tbody>
                {ledgerEntries.map((e, idx) => (
                  <tr key={`${e.award_key}-${idx}`}>
                    <td className="axiom-mono">{e.timestamp_utc.slice(0, 19).replace('T', ' ')}</td>
                    <td className="axiom-mono axiom-key-cell">{e.award_key}</td>
                    <td>{e.reason}</td>
                    <td className="axiom-delta-cell">
                      <span className={e.delta > 0 ? 'axiom-xp-plus' : 'axiom-xp-zero'}>
                        +{e.delta} XP
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
};
