# Graph API

## Introduction

The Graph API is an internal interface that sits on top of the oscoin protocol, and gives read and write access to the oscoin "world state". The primary consumers of this API are the oscoin **ledger** and **osrank**.

In this document, we define the Graph API as a set of **rust traits**, to be imported by the ledger and osrank projects, and implemented by the **protocol**.
When we speak of *ledger*, we refer to blockchain's domain logic and state
machine. This includes any underlying infrastructure required for the ledger to
operate.

We explain how this API is intended to be used, and how the different consumers integrate with the protocol via this API.

## Network graph construction

It's the **ledger** that is responsible for constructing the "Network" graph - the graph of dependencies and contributions. To do this, the `GraphWriter` trait is provided, which allows for the adding and removing of nodes and edges. The oscoin domain is mapped to a graph as follows:

      Network Graph   =>   Graph
      Contributor     =>   Node
      Project         =>   Node
      Dependency      =>   Edge
      Commit          =>   Edge
      Maintenance     =>   Edge
      Osrank          =>   GraphAlgorithm (algorithm) /
                           GraphAnnotator (result)

The ledger operations are roughly mapped as follows:

      RegisterProject    => GraphWriter::add_node
      UnregisterProject  => GraphWriter::remove_node
      Checkpoint         => GraphWriter::{add_node, add_edge, remove_edge}
      AddMaintainer      => GraphWriter::add_edge
      RemoveMaintainer   => GraphWriter::remove_edge

Note that value transfer is intended to be done at the protocol level.

If we follow an entire oscoin life-cycle through the GraphAPI, we get something like this:

1. A `Checkpoint` transaction comes in through the node and is handed to the ledger.
2. The ledger adds edges for new dependencies and removes edges for stale dependencies, via `GraphWriter`, provided as a WASM import.[^1]
3. When osrank runs for the first time, `GraphAlgorithm::execute` is called with an empty context (`GraphAlgorithm::Context::default()`), and an input `GraphAnnotator`. The osrank algorithm is free to store any state that will be useful for the next run into the input `Context`. This plays the role of a cache that is accessible through subsequent runs. For example, the random walks can be stored in the case of an incremental algorithm.
5. Osrank's implementation of `GraphAlgorithm` traverses the graph via the `Graph` trait and annotates nodes with an osrank via `GraphAnnotator`.
6. When the algorithm finishes, it *may* return an output which is stored in the "World State".
7. On subsequent runs of `GraphAlgorithm::execute`, the previous `Context` is passed in.


         L I G H T     L E D G E R <---.     O S R A N K -- GraphAlgorithm
        C L I E N T S       |          |          |           |  |
             |              |          |          |           |  |
             |          GraphWriter    |     GraphAnnotator --'  |
             |              |          '------.   |              |
             |        .-----|------------.    |   |              |
             |        |     | WASMi      |    |   |              |
       P .---v--------+-----v------------|    |   |              |
       R |  LedgerAPI |   GraphAPI       <----)---'              |
       O |------------+---.--------------|    |                  |
       T |  World State   |   Context    |   eval                |
       O |----------------'--------------|   /                   |
       C |        Block execution        |__/                    |
       O |-------------------------------|  \__execute___________|
       L |          Consensus            |
         '-------------------------------'


                  Ledger:            Oscoin domain logic & semantics
                  LedgerAPI:         External oscoin API for light clients
                  WASMi:             Web Assembly interpreter
                  GraphAPI:          What this document is about
                  WorldState:        Materialized public blockchain state
                  Context:           Context available to the GraphAlgorithm
                  Block execution:   Executes blocks and distributes rewards
                  Consensus:         Agrees on blocks

## Traits

The following traits compose the Graph API:

### ``GraphAPI``

The top level API which gives access to all relevant objects.

### ``Graph``

A read-only graph.

### ``GraphWriter``

A writable graph. Used by the ledger to add/remove nodes and edges when constructing the dependency graph.

### ``GraphDataWriter``

A graph with writable node and edge data. This API doesn't allow adding and removing of nodes or edges. Used by the ledger to update edge data, such as upon a user's second or third contribution to a project.

### `GraphAnnotator`

An object which allows writing a single type of data to to the graph. Used by osrank to write the project rankings.

### ``GraphObject``

An object in the graph, eg. a node or edge.

### ``GraphAlgorithm``

An algorithm that can run on a ``GraphDataWriter`` and update graph data.

### ``Node``

A node or "vertex" in the graph. For the purpose of oscoin, these will mainly represent projects.

### ``Edge``

A link between two nodes in the graph. For the purpose of oscoin, these will mainly represent dependencies between projects, or contributions from users to projects.

---

## Questions

### Answered

How do the Ledger and Osrank integrate on the graph? Eg. how does the Ledger encode contributions such that Osrank can decode them?

> We use static types for `NodeData` and `EdgeData` that are shared across the two systems. This ensures that the data stored by the Ledger can be understood by Osrank.

Where do optimizations lie, when it comes to Osrank?

> Any Osrank optimization, for eg. when it comes to caching or more efficient representations of data - are handled by Osrank and/or the underlying storage system, and are not exposed at the API level. The API should stay focused on providing the simplest abstractions to use.

How is the data in the Graph accessed by light clients?

> The data is accessed through the "Ledger API" which gives access to the "World State" via a key/value interface. Clients never write to the graph directly, but only through transactions processed by the Ledger.

Can "unclaimed" projects receive osrank and payout?

> For now, unclaimed projects are simply treated as non-existent projects and are ignored.

### Unanswered

* Do we need a `GraphObject` type?
* How does the Ledger access the account balance of a project or user?
* What is the interface through which Osrank receives graph updates?
* Can "unclaimed" contributions receive osrank and payout?
* How exactly does the protocol call into the ledger and distribute payouts?
* How are funds distributed within a project?
