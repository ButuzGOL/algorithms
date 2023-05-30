class Node {
  constructor(value) {
    this.value = value;
    this.left = null;
    this.right = null;
  }
}

class BinarySearchTree {
  constructor() {
    this.root = null;
  }
  insert(val) {
    function walk(node) {
      if (val > node.value) {
        if (!node.right) node.right = new Node(val);
        else walk(node.right);
      } else {
        if (!node.left) node.left = new Node(val);
        else walk(node.left);
      }
    }
    walk(this.root);
  }
  find(val) {
    let founded;
    function walk(node) {
      if (node.value === val) founded = node;
      if (founded !== undefined) return;
      if (val > node.value) {
        if (!node.right) founded = null;
        else walk(node.right);
      } else {
        if (!node.left) founded = null;
        else walk(node.left);
      }
    }
    walk(this.root);

    return founded;
  }
  BFS() {
    const data = [];
    const queue = [this.root];
    while (queue.length) {
      const curr = queue.shift();
      data.push(curr);
      if (curr.left) {
        queue.push(curr.left);
      }
      if (curr.right) {
        queue.push(curr.right);
      }
    }

    return data;
  }
  DFSPreOrder() {
    const data = [];
    function walk(node) {
      data.push(node.value);
      if (node.left) walk(node.left);
      if (node.right) walk(node.right);
    }
    walk(this.root);

    return data;
  }

  DFSPostOrder() {
    const data = [];
    function walk(node) {
      if (node.left) walk(node.left);
      if (node.right) walk(node.right);
      data.push(node.value);
    }
    walk(this.root);

    return data;
  }
  DFSInOrder() {
    const data = [];
    function walk(node) {
      if (node.left) walk(node.left);
      data.push(node.value);
      if (node.right) walk(node.right);
    }
    walk(this.root);

    return data;
  }
}

var tree = new BinarySearchTree();
tree.root = new Node(10);
tree.root.right = new Node(15);
tree.root.right.right = new Node(20);
tree.root.left = new Node(6);
tree.root.left.left = new Node(3);
tree.root.left.right = new Node(8);

// tree.insert(13);

console.log(tree);

tree.find(10);
tree.BFS();
tree.DFSPreOrder();
tree.DFSPostOrder();
tree.DFSInOrder();
