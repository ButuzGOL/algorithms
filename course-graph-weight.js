class PriorityQueue {
  constructor() {
    this.values = [];
  }
  enqueue(val, priority) {
    this.values.push({ val, priority });
    this.sort();
  }
  dequeue() {
    return this.values.shift();
  }
  sort() {
    this.values.sort((a, b) => a.priority - b.priority);
  }
}

class WeightedGraph {
  constructor() {
    this.adjacencyList = {};
  }
  addVertex(vertex) {
    if (!this.adjacencyList[vertex]) this.adjacencyList[vertex] = [];
  }
  addEdge(v1, v2, weight) {
    this.adjacencyList[v1].push({ n: v2, w: weight });
    this.adjacencyList[v2].push({ n: v1, w: weight });
  }
  Dijkstra(start, finish) {
    const visited = {};
    const previous = {};
    const startVertex = start;
    let currentVertexes = [{ n: startVertex, w: 0 }];
    visited[startVertex] = true;
    while (currentVertexes.length) {
      const vertex = currentVertexes.shift();
      visited[vertex.n] = true;

      let pushed = [];
      for (let i = 0; i < this.adjacencyList[vertex.n].length; i++) {
        const item = this.adjacencyList[vertex.n][i];

        if (!visited[item.n]) {
          pushed.push(item);
        }
      }

      const filterInx = [];
      for (let i = 0; i < pushed.length; i++) {
        const we = vertex.w + pushed[i].w;
        if (
          previous[pushed[i].n] === undefined ||
          we < previous[pushed[i].n].w
        ) {
          previous[pushed[i].n] = { n: vertex.n, w: we };
        } else {
          filterInx.push(i);
        }
        pushed[i] = { n: pushed[i].n, w: we };
      }

      pushed = pushed.filter((itm, inx) => !filterInx.includes(inx));

      for (let i = 0; i < pushed.length; i++) {
        currentVertexes.push(pushed[i]);
      }
      currentVertexes.sort((a, b) => a.w - b.w);
    }

    return previous;
  }
  Dijkstra1(start, finish) {
    const nodes = new PriorityQueue();
    const distances = {};
    const previous = {};
    let path = []; //to return at end
    let smallest;
    //build up initial state
    for (let vertex in this.adjacencyList) {
      if (vertex === start) {
        distances[vertex] = 0;
        nodes.enqueue(vertex, 0);
      } else {
        distances[vertex] = Infinity;
        nodes.enqueue(vertex, Infinity);
      }
      previous[vertex] = null;
    }
    // as long as there is something to visit
    while (nodes.values.length) {
      smallest = nodes.dequeue().val;
      if (smallest === finish) {
        //WE ARE DONE
        //BUILD UP PATH TO RETURN AT END
        while (previous[smallest]) {
          path.push(smallest);
          smallest = previous[smallest];
        }
        break;
      }
      if (smallest || distances[smallest] !== Infinity) {
        for (let neighbor in this.adjacencyList[smallest]) {
          //find neighboring node
          let nextNode = this.adjacencyList[smallest][neighbor];
          //calculate new distance to neighboring node
          let candidate = distances[smallest] + nextNode.w;
          let nextNeighbor = nextNode.n;
          if (candidate < distances[nextNeighbor]) {
            //updating new smallest distance to neighbor
            distances[nextNeighbor] = candidate;
            //updating previous - How we got to neighbor
            previous[nextNeighbor] = smallest;
            //enqueue in priority queue with new priority
            nodes.enqueue(nextNeighbor, candidate);
          }
        }
      }
    }
    return path.concat(smallest).reverse();
  }
}

let g = new WeightedGraph();
g.addVertex("A");
g.addVertex("B");
g.addVertex("C");
g.addVertex("D");
g.addVertex("E");
g.addVertex("F");

g.addEdge("A", "B", 4);
g.addEdge("A", "C", 2);
g.addEdge("B", "E", 3);
g.addEdge("C", "D", 2);
g.addEdge("C", "F", 4);
g.addEdge("D", "E", 3);
g.addEdge("D", "F", 1);
g.addEdge("E", "F", 1);

g.Dijkstra("A", "E");
g.Dijkstra1("A", "E");
