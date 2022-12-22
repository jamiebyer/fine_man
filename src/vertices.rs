
enum 2PtVertex {
    2PtVertex
}

struct 3PtLoop {
    v1: 3PtVertex,
    v2: 3PtVertex,
    v3: 3PtVertex
}

struct 3PtBranchLoop {
    v1: 3PtVertex,
    orientation: String
}

enum 3PtVertex {
    3PtTree,
    3PtLoop(3PtLoop),
    3PtBranchLoop(3PtBranchLoop)
}

struct 4PtLoop {
    v1: 3PtVertex,
    v2: 3PtVertex,
    orientation: String
}

struct 4PtLoop {
    v1: 4PtVertex,
    v2: 4PtVertex,
    orientation: String
}

enum 4PtVertex {
    4PtTree, // X
    4PtLine(4PtLine), // standard diagram shape
    4PtLoop(4PtLoop),
}