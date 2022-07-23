const fs = require("fs");

const splitData = (data) => {
  data = data.toString();
  data = data.split("\n");
  return data;
}

const formatData = (data) => {
  return data.map((d) => {
    const details = d.split(" ");
    let dir = details[0];
    let am = details[1];

    return {
      "direction": dir,
      "amount": am
    }
  });
}

const increaseValue = (curr, increase) => {
  return curr + increase;
}

const decreaseValue = (curr, decrease) => {
  return curr - decrease;
}

const mutliplyValues = (curr, increase) => {
  return curr * increase;
}

const getSubPosition = (data) => {
  let horizontal = 0;
  let depth = 0;
  let aim = 0;

  data.forEach((movement) => {
    const dir = movement.direction.toString();
    const am = Number(movement.amount);

    switch(dir) {
      case "forward": 
        horizontal = increaseValue(horizontal, am);
        depth = increaseValue(depth, mutliplyValues(aim, am));
      break;

      case "down": 
        aim = increaseValue(aim, am);
      break;

      case "up": 
        aim = decreaseValue(aim, am);
      break;
    }
  });

  return horizontal * depth;
}

const main = () => {
  fs.readFile("../input/dive_input.txt", "utf-8", (err, d) => {
    if (err) throw err;

    const split = splitData(d);
    const data = formatData(split);
    const result = getSubPosition(data);
    console.log(result);
  });
}

main();
