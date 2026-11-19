-- AXIOM Mathematics Academy — Phase 2 Backend Durability & Evidence Pipeline
-- Governed by docs/AXIOM-SOURCE-OF-TRUTH.md Section 21 & 22

-- Server-side mathematical verification receipts for cryptographic audit trails
CREATE TABLE IF NOT EXISTS evidence_verifications (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  instance_id uuid NOT NULL,
  attempt_id uuid NOT NULL,
  problem_hash text NOT NULL CHECK (problem_hash ~ '^[0-9a-f]{64}$'),
  submission_hash text NOT NULL CHECK (submission_hash ~ '^[0-9a-f]{64}$'),
  verified_disposition text NOT NULL CHECK (verified_disposition IN ('correct', 'incorrect', 'malformed', 'unsupported', 'inconclusive', 'pending_review')),
  verified_score integer NOT NULL CHECK (verified_score >= 0 AND verified_score <= 100),
  checker_version text NOT NULL,
  policy_version text NOT NULL,
  evidence_hash text NOT NULL CHECK (evidence_hash ~ '^[0-9a-f]{64}$'),
  raw_evidence jsonb NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  UNIQUE (attempt_id, checker_version),
  FOREIGN KEY (attempt_id, user_id) REFERENCES attempts(id, user_id)
);

-- Client account durable backup and synchronization snapshots
CREATE TABLE IF NOT EXISTS account_backups (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  snapshot_hash text NOT NULL CHECK (snapshot_hash ~ '^[0-9a-f]{64}$'),
  client_event_id uuid NOT NULL,
  policy_version text NOT NULL DEFAULT '1.1.0',
  entry_count integer NOT NULL CHECK (entry_count >= 0),
  payload jsonb NOT NULL,
  restored_at timestamptz,
  created_at timestamptz NOT NULL DEFAULT now(),
  UNIQUE (user_id, snapshot_hash),
  UNIQUE (user_id, client_event_id)
);

-- Performance and deduplication indexes
CREATE INDEX IF NOT EXISTS idx_evidence_user_attempt ON evidence_verifications (user_id, attempt_id);
CREATE INDEX IF NOT EXISTS idx_evidence_problem_hash ON evidence_verifications (problem_hash, verified_disposition);
CREATE INDEX IF NOT EXISTS idx_backups_user_created ON account_backups (user_id, created_at DESC);
