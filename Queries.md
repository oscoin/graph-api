# Queries to the ledger state

This document describes how user queries map to the ledger state.

---

Keys in the merkleized state are all *binary*, but for readability,
we describe them as ASCII, with `/` as the delimiter.

All queries are range queries, but in cases where a single value
is of interest, the range contains only one key.

Queries marked *N/A* are not available in a merkleized form.
Queries marked *???* need more research.

## Basic

* Get account with balance & nonce:

    get /accounts/:id -> Account

* Get project metadata, including osrank, contract type and balance:

    get /projects/:id -> Project

* Get project members (range):

    get /projects/:id/members/{0..n} -> [AccountId]

* Get specific checkpoint hash of project:

    get /projects/:id/checkpoints/:index -> Hash

* Get latest checkpoint hash of project:

    get /projects/:id/checkpoints/latest -> Hash

* Get the first `n` checkpoints of a project:

    get /projects/:id/checkpoints/{0..n} -> [Hash]

* Get checkpoint version or other metadata, given the hash:

    get /checkpoints/:hash -> Checkpoint

* How much rewards did a project get in the given epoch?

    get /projects/:id/rewards/:epoch -> Balance

* How much rewards in total did a project get?

    get /projects/:id/rewards/{0..-1} -> [Balance]

  And sum the returned balances.

## Names

* Resolve "acme", returning the public key it resolves to and TTL:

    get /names/acme -> NameEntry

## Graph Data

* Get all checkpointed contributions to a project:

    get /projects/:id/contributions/{0..-1} -> [Contribution]

* Check whether someone is a verified contributor to a project, given
  their account id and a contribution index:

  For someone to be considered "verified", they have to verify
  their GPG key on-chain.

    get /projects/:id/contributions/:index -> Contribution { gpgkey }
    get /keys/:gpgkey -> AccountId

* Get all registered projects that project <id> depends on:

    get /projects/:id/dependencies/{0..-1} -> [(ProjectId, Hash)]

  Returns the project ids along with the checkpoint hash which
  contains the version depended on.

* What registered projects depend on project <id>?

*N/A*

* Who donated to project <id>?

    get /projects/:id/donations/{0..-1} -> [(AccountId, Balance)]

* What are all the rewards received by project <id> and how were they redistributed?

*???*

## Reverse lookups

These lookups are not possible via the Merkle state, and should be
handled locally by the client.

* What projects am I a member of?

*N/A*

* What projects have I contributed to?

*N/A*

* What names do I own?

*N/A*
