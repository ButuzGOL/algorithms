class Node {
  constructor(val, next) {
    this.val = val;
    this.next = next;
    this.prev = null;
  }
}

class SingleLinkedList {
  constructor() {
    this.head = null;
    this.tail = null;
    this.length = 0;
  }
  push(val) {
    const node = new Node(val);
    if (!this.head) {
      this.head = node;
      this.tail = this.head;
    } else {
      this.tail.next = node;
      this.tail = node;
    }

    this.length++;
    return this;
  }
  pop() {
    const oldTail = this.tail;
    if (this.length === 1) {
      this.tail = null;
      this.head = null;
    } else {
      let next = this.head;
      while (next.next.next) {
        next = next.next;
      }
      this.tail = next;
      this.tail.next = null;
    }

    this.length--;
    return oldTail;
  }
  shift() {
    const oldTail = this.head;
    if (this.length === 1) {
      this.tail = null;
      this.head = null;
    } else {
      this.head = this.head.next;
    }

    this.length--;
    return oldTail;
  }
  unshift(val) {
    const node = new Node(val);
    node.next = this.head;
    this.head = node;
    this.length++;
    return this;
  }
  get(inx) {
    let inInx = 0;
    let next = this.head;
    while (inInx !== inx) {
      next = next.next;
      inInx++;
    }

    return next;
  }
  set(inx, val) {
    const node = this.get(inx);
    node.val = val;
    return node;
  }
  print() {
    let next = this.head;
    const list = [];
    while (next.next) {
      list.push(next.val);
      next = next.next;
    }
    list.push(next.val);
    return list;
  }
  reverse() {
    let node = this.head;
    let prev;
    let next = node;

    const oldHead = this.head;
    this.head = this.tail;
    this.tail = oldHead;

    for (let i = 0; i < this.length; i++) {
      node = next;
      next = node.next;
      node.next = prev;
      prev = node;
    }
  }
}

const list = new SingleLinkedList();
list.push("1");
list.push("2");
list.push("3");

// console.log(list.print());
// list.pop();
// list.unshift("-1");
// list.get(0);
// list.reverse();
// console.log(list.print());
// console.log(list);

class DoublyLinkedList {
  constructor() {
    this.head = null;
    this.tail = null;
    this.length = 0;
  }
  push(val) {
    var node = new Node(val);
    if (this.head === null) {
      this.head = node;
      this.tail = this.head;
      this.length++;
    } else {
      this.tail.next = node;
      node.prev = this.tail;
      this.tail = node;
      this.length++;
    }
    return this;
  }
  print() {
    let next = this.head;
    const list = [];
    while (next.next) {
      list.push(next.val);
      next = next.next;
    }
    list.push(next.val);
    return list;
  }
  reverse() {
    let node = this.head;

    const oldHead = this.head;
    this.head = this.tail;
    this.tail = oldHead;

    let next = node;
    for (let i = 0; i < this.length; i++) {
      node = next;
      next = next.next;
      let next1 = node.next;
      node.next = node.prev;
      node.prev = next1;
    }
  }
}

let doublyLinkedList = new DoublyLinkedList();
doublyLinkedList.push(5).push(10).push(15).push(20);
doublyLinkedList.reverse(); // singlyLinkedList;
console.log(doublyLinkedList.print());
// list1.reverse();
// console.log(list1.print());

function hash(key, arrayLen) {
  let total = 0;
  for (let char of key) {
    let value = char.charCodeAt(0) - 96;
    total = (total + value) % arrayLen;
  }
  return total;
}

hash("apink", 10);
