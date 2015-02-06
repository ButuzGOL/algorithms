var events = [
  { start: 0, end: 30 },
  { start: 30, end: 100 },
  { start: 80, end: 160 },
  { start: 110, end: 240 },
  { start: 110, end: 180 },
  { start: 170, end: 230 },
  { start: 170, end: 240 },
  { start: 170, end: 220 },
  { start: 190, end: 260 },
  { start: 230, end: 300 },
  { start: 240, end: 280 },
  { start: 250, end: 290 },
  { start: 300, end: 330 }
];

function build(events) {
  var numColumns = 0;
  var eventGroups = [];
  var group = { end: 0 };
  
  events.sort(function(a, b) {
    if (a.start > b.start) return 1;
    if (a.start < b.start) return -1;
    if (a.end > b.end) return 1;
    if (a.end < b.end) return -1;

    return 0;
  });

  events.forEach(function(event, index) {
    if (event.start >= group.end || !eventGroups.length) {
      eventGroups.push({ columns: [{ end: 0 }], end: 0 });
      group = eventGroups[eventGroups.length - 1];
      numColumns = 0;
    }

    if (event.end > group.end) {
      group.end = event.end;
    }

    event.column = numColumns;

    group.columns.forEach(function(column, index) {
      if (event.start >= column.end && numColumns) {
        event.column = index;
      }
    });

    if (event.column === numColumns) {
      numColumns++;
    }

    group.columns[event.column] = {
      end: event.end
    };

    event.group = group;
  });

  events.forEach(function(event) {
    event.top = event.start;
    event.height = event.end - event.start;

    event.width = 600 / event.group.columns.length;
    event.left = event.column * event.width;
  });

  return events;
}

function draw(events) {
  var fullWidth = 80;
  var lines = [];
  var max = 0;
  var numberOffset;

  events.forEach(function(event) {
    if (event.end > max) {
      max = event.end;
    }
  });

  events.sort(function(a, b) {
    return a.column - b.column;
  });

  numberOffset = max.toString().length;

  for(var i = 10; i <= max; i += 10) {
    var line = i;
    var pointer = 0;

    for(var t = 0; t < (numberOffset - i.toString().length); t++) {
      line += ' ';
    }

    line += ' |';
    
    events.forEach(function(event) {
      var symbol = ' ';
      var width = Math.ceil(fullWidth / event.group.columns.length);
      var left = Math.ceil(event.column * width);
      var j;

      if (event.start < i && event.end >= i) {
        if (left && left > pointer) {
          for(j = 0; j < ((pointer < 0) ? left: (left - pointer)); j++) {
            line += ' ';
          }
        }
        line += '|';
      } else {
        return;
      }

      if (event.end === i) {
        symbol = '_';
      }

      for(j = 0; j < width - 2; j++) {
        line += symbol;
      }

      line += '|';

      pointer = line.length - (line.indexOf('|') + 1);
    });

    lines.push(line);
  }

  console.log(lines.join('\n'));
}

// draw(build(events));
//
// 10  ||                                                                              |
// 20  ||                                                                              |
// 30  ||______________________________________________________________________________|
// 40  ||              |
// 50  ||              |
// 60  ||              |
// 70  ||              |
// 80  ||              |
// 90  ||              ||              |
// 100 ||______________||              |
// 110 |                |              |
// 120 ||              ||              ||              |
// 130 ||              ||              ||              |
// 140 ||              ||              ||              |
// 150 ||              ||              ||              |
// 160 ||              ||______________||              |
// 170 ||              |                |              |
// 180 ||______________||              ||              ||              ||              |
// 190 |                |              ||              ||              ||              |
// 200 ||              ||              ||              ||              ||              |
// 210 ||              ||              ||              ||              ||              |
// 220 ||              ||______________||              ||              ||              |
// 230 ||              |                |              ||______________||              |
// 240 ||              |                |______________||              ||______________|
// 250 ||              |                                |              ||              |
// 260 ||______________|                |              ||              ||              |
// 270 |                                |              ||              ||              |
// 280 |                                |              ||              ||______________|
// 290 |                                |______________||              |
// 300 |                                                |______________|
// 310 ||                                                                              |
// 320 ||                                                                              |
// 330 ||______________________________________________________________________________|
