function getInput() {
  return "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
}

function parseLine(line: string): [number, number] {
  const [dir, a] = line.split(" ");
  const amount = +a;

  if (dir === "forward") {
    return [amount, 0];
  } else if (dir === "down") {
    return [0, amount];
  }
  return [0, -amount];
}

const items = getInput()
  .split("\n")
  .map(parseLine)
  .reduce(
    (acc, [x, y]) => {
      acc[0] += x;
      acc[1] += y;
      return acc;
    },
    [0, 0]
  );

console.log(items, Math.abs(items[0]) * Math.abs(items[1]));
