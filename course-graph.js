class Graph {
  constructor() {
    this.adjacencyList = {};
  }
  addVertex(vertex) {
    if (!this.adjacencyList[vertex]) this.adjacencyList[vertex] = [];
  }
  addEdge(v1, v2) {
    this.adjacencyList[v1].push(v2);
    this.adjacencyList[v2].push(v1);
  }
  removeEdge(vertex1, vertex2) {
    this.adjacencyList[vertex1] = this.adjacencyList[vertex1].filter(
      (v) => v !== vertex2
    );
    this.adjacencyList[vertex2] = this.adjacencyList[vertex2].filter(
      (v) => v !== vertex1
    );
  }
  removeVertex(vertex) {
    while (this.adjacencyList[vertex].length) {
      const adjacentVertex = this.adjacencyList[vertex].pop();
      this.removeEdge(vertex, adjacentVertex);
    }
    delete this.adjacencyList[vertex];
  }
  DFR(startVertex = "A") {
    const arr = [];
    const visited = {};

    const self = this;
    function walk(vertex) {
      visited[vertex] = true;
      arr.push(vertex);
      for (let i = 0; i < self.adjacencyList[vertex].length; i++) {
        if (visited[self.adjacencyList[vertex][i]]) continue;
        walk(self.adjacencyList[vertex][i]);
      }
    }
    walk(startVertex);

    return arr;
  }
  DFL(startVertex = "A") {
    const arr = [];
    const visited = {};

    let currentVertexes = [startVertex];
    visited[startVertex] = true;
    while (currentVertexes.length) {
      const vertex = currentVertexes.pop();
      arr.push(vertex);

      for (let i = 0; i < this.adjacencyList[vertex].length; i++) {
        if (!visited[this.adjacencyList[vertex][i]]) {
          visited[this.adjacencyList[vertex][i]] = true;
          currentVertexes.push(this.adjacencyList[vertex][i]);
        }
      }
    }

    return arr;
  }
  BFS(startVertex = "A") {
    const arr = [];
    const visited = {};

    let currentVertexes = [startVertex];
    visited[startVertex] = true;
    while (currentVertexes.length) {
      const vertex = currentVertexes.shift();
      arr.push(vertex);

      for (let i = 0; i < this.adjacencyList[vertex].length; i++) {
        if (!visited[this.adjacencyList[vertex][i]]) {
          visited[this.adjacencyList[vertex][i]] = true;
          currentVertexes.push(this.adjacencyList[vertex][i]);
        }
      }
    }

    return arr;
  }
}

let g = new Graph();
g.addVertex("A");
g.addVertex("B");
g.addVertex("C");
g.addVertex("D");
g.addVertex("E");
g.addVertex("F");

g.addEdge("A", "B");
g.addEdge("A", "C");
g.addEdge("B", "D");
g.addEdge("C", "E");
g.addEdge("D", "E");
g.addEdge("D", "F");
g.addEdge("E", "F");
g.DFR();
g.DFL();

console.log("A");
