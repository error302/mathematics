-- AXIOM Mathematics Academy — Core Relational Schema
-- Conforms strictly to AXIOM-SOURCE-OF-TRUTH.md Section 21

CREATE TABLE users (
  id uuid PRIMARY KEY,
  status text NOT NULL CHECK (status IN ('active', 'disabled', 'deletion_pending')),
  locale text NOT NULL DEFAULT 'en',
  timezone text NOT NULL DEFAULT 'UTC',
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE external_identities (
  provider text NOT NULL,
  subject text NOT NULL,
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (provider, subject),
  UNIQUE (user_id, provider)
);

CREATE TABLE sessions (
  token_hash text PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  expires_at timestamptz NOT NULL,
  revoked boolean NOT NULL DEFAULT false,
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE learning_streams (
  user_id uuid PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
  next_sequence bigint NOT NULL DEFAULT 1 CHECK (next_sequence > 0)
);

CREATE TABLE exercise_instances (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  template_id text NOT NULL,
  template_version text NOT NULL,
  generator_version text NOT NULL,
  checker_version text NOT NULL,
  manifest_hash text NOT NULL CHECK (manifest_hash ~ '^[0-9a-f]{64}$'),
  seed_hex text NOT NULL CHECK (seed_hex ~ '^[0-9a-f]{64}$'),
  context text NOT NULL CHECK (context IN ('practice', 'assessment', 'review')),
  problem_hash text NOT NULL CHECK (problem_hash ~ '^[0-9a-f]{64}$'),
  artifact jsonb NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  UNIQUE (id, user_id)
);

CREATE TABLE attempts (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  instance_id uuid NOT NULL,
  origin text NOT NULL CHECK (origin IN ('online', 'offline', 'guest_import')),
  client_event_id uuid NOT NULL,
  submission_hash text NOT NULL CHECK (submission_hash ~ '^[0-9a-f]{64}$'),
  answer jsonb NOT NULL,
  assistance jsonb NOT NULL,
  received_at timestamptz NOT NULL DEFAULT now(),
  UNIQUE (id, user_id),
  UNIQUE (user_id, client_event_id),
  FOREIGN KEY (instance_id, user_id) REFERENCES exercise_instances(id, user_id)
);

CREATE TABLE grade_results (
  id uuid PRIMARY KEY,
  attempt_id uuid NOT NULL,
  user_id uuid NOT NULL,
  ordinal integer NOT NULL CHECK (ordinal > 0),
  disposition text NOT NULL CHECK (disposition IN ('correct', 'incorrect', 'malformed', 'unsupported', 'inconclusive', 'pending_review')),
  checker_version text NOT NULL,
  policy_version text NOT NULL,
  evidence jsonb NOT NULL,
  evaluated_at timestamptz NOT NULL DEFAULT now(),
  FOREIGN KEY (attempt_id, user_id) REFERENCES attempts(id, user_id),
  UNIQUE (attempt_id, ordinal),
  UNIQUE (id, attempt_id, user_id)
);

CREATE TABLE current_grades (
  attempt_id uuid PRIMARY KEY,
  user_id uuid NOT NULL,
  grade_id uuid NOT NULL,
  revision bigint NOT NULL CHECK (revision > 0),
  FOREIGN KEY (attempt_id, user_id) REFERENCES attempts(id, user_id),
  FOREIGN KEY (grade_id, attempt_id, user_id) REFERENCES grade_results(id, attempt_id, user_id)
);

CREATE TABLE learning_events (
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  sequence bigint NOT NULL CHECK (sequence > 0),
  event_id uuid NOT NULL,
  event_type text NOT NULL,
  schema_version integer NOT NULL CHECK (schema_version > 0),
  payload jsonb NOT NULL,
  occurred_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (user_id, sequence),
  UNIQUE (user_id, event_id)
);

CREATE TABLE reward_entries (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  award_key text NOT NULL,
  source_sequence bigint NOT NULL,
  policy_version text NOT NULL,
  delta integer NOT NULL,
  reason text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  UNIQUE (user_id, award_key),
  FOREIGN KEY (user_id, source_sequence) REFERENCES learning_events(user_id, sequence)
);

CREATE TABLE mastery_projections (
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  concept_id text NOT NULL,
  dimension text NOT NULL,
  alpha double precision NOT NULL,
  beta double precision NOT NULL,
  evidence_count bigint NOT NULL DEFAULT 0,
  state text NOT NULL,
  revision bigint NOT NULL DEFAULT 1,
  updated_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (user_id, concept_id, dimension)
);

CREATE TABLE review_schedules (
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  concept_id text NOT NULL,
  step integer NOT NULL DEFAULT 0,
  due_at timestamptz NOT NULL,
  updated_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (user_id, concept_id)
);

CREATE TABLE notebook_documents (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  doc_type text NOT NULL,
  title text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE notebook_revisions (
  id uuid PRIMARY KEY,
  document_id uuid NOT NULL REFERENCES notebook_documents(id) ON DELETE CASCADE,
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  parent_revision_id uuid,
  content_hash text NOT NULL,
  content jsonb NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE outbox_jobs (
  id uuid PRIMARY KEY,
  job_type text NOT NULL,
  payload jsonb NOT NULL,
  dedup_key text UNIQUE,
  lease_token text,
  lease_expires_at timestamptz,
  attempts integer NOT NULL DEFAULT 0,
  max_attempts integer NOT NULL DEFAULT 5,
  status text NOT NULL CHECK (status IN ('pending', 'leased', 'completed', 'failed', 'dead_letter')),
  next_retry_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now()
);

-- Core performance and uniqueness indexes (Section 21.2)
CREATE INDEX idx_attempts_user_received ON attempts (user_id, received_at DESC, id);
CREATE INDEX idx_instances_user_context ON exercise_instances (user_id, context, created_at DESC);
CREATE INDEX idx_mastery_user ON mastery_projections (user_id, concept_id);
CREATE INDEX idx_reviews_due ON review_schedules (user_id, due_at ASC);
CREATE INDEX idx_outbox_pending ON outbox_jobs (status, next_retry_at ASC) WHERE status IN ('pending', 'leased');
CREATE INDEX idx_events_user_seq ON learning_events (user_id, sequence ASC);
