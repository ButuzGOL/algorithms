var events = [
  { start: 0, end: 30 },
  { start: 30, end: 100 },
  { start: 80, end: 160 },
  { start: 110, end: 240 },
  { start: 110, end: 240 },
  { start: 110, end: 180 },
  { start: 170, end: 230 },
  { start: 170, end: 240 },
  { start: 170, end: 220 },
  { start: 190, end: 260 },
  { start: 230, end: 300 },
  { start: 240, end: 280 },
  { start: 250, end: 290 },
  { start: 300, end: 330 },
  { start: 330, end: 400 },
  { start: 330, end: 390 },
  { start: 330, end: 380 },
  { start: 400, end: 500 },
  { start: 400, end: 620 },
  { start: 600, end: 650 },
  { start: 660, end: 720 },
  { start: 670, end: 720 },
  { start: 680, end: 720 },
  { start: 690, end: 720 },
  { start: 650, end: 680 },
  { start: 650, end: 690 } 
];

function calendar(events) {
  var numColumns = 0;
  var eventGroups = [];
  var group = { end: 0 };
  var fullWidth = 600;

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
      if (event.start > column.end && numColumns) {
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

    event.width = fullWidth / event.group.columns.length;
    event.left = event.column * event.width;
  });

  return events;
}

console.log(calendar(events));
