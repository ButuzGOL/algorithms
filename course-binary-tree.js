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
}

var tree = new BinarySearchTree();
tree.root = new Node(10);
tree.root.right = new Node(15);
tree.root.right.right = new Node(20);
tree.root.left = new Node(7);
tree.root.left.right = new Node(9);

tree.insert(13);

console.log(tree);

tree.find(10);
