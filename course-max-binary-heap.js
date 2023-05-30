class MaxBinaryHeap {
  constructor() {
    this.values = [];
  }
  insert(val) {
    this.values.push(val);
    let pInx = this.values.length - 1;
    let nInx = Math.floor((pInx - 1) / 2);
    // 33 41
    // (n - 1) / 2
    // 6 => (5 - 1) / 2 => 2
    // 2 => (2 - 1) / 2 => 0.5
    let next = true;
    while (next) {
      next = false;
      if (this.values[nInx] < val) {
        let tmp = this.values[pInx];
        this.values[pInx] = this.values[nInx];
        this.values[nInx] = tmp;
        next = true;
      }
      pInx = nInx;
      nInx = Math.floor((nInx - 1) / 2);
    }
    return this.values;
  }
  extractMax() {
    const swap = (i0, i1) => {
      let tmp = this.values[i0];
      this.values[i0] = this.values[i1];
      this.values[i1] = tmp;
    };
    let last = this.values.pop();
    this.values[0] = last;

    let inx = 0;
    let lChildInx = 2 * inx + 1;
    let rChildInx = 2 * inx + 2;
    while (true) {
      if (this.values[inx] < this.values[lChildInx]) {
        if (this.values[inx] < this.values[rChildInx]) {
          if (this.values[lChildInx] > this.values[rChildInx]) {
            swap(inx, lChildInx);
            inx = lChildInx;
          } else {
            swap(inx, rChildInx);
            inx = rChildInx;
          }
        } else {
          swap(inx, lChildInx);
        }
      } else if (this.values[inx] > this.values[rChildInx]) {
        swap(inx, rChildInx);
        inx = rChildInx;
      } else {
        break;
      }
      rChildInx = 2 * lChildInx + 2;
      lChildInx = 2 * lChildInx + 1;
    }

    return this.values;
  }
}

const maxBinaryHeap = new MaxBinaryHeap();
maxBinaryHeap.values = [41, 39, 33, 18, 27, 12];
// maxBinaryHeap.insert(55);
maxBinaryHeap.extractMax();
