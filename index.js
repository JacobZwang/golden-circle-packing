var c = document.getElementById("canvas");
var ctx = c.getContext("2d");

async function getData() {
  const data = await fetch(
    "http://localhost:5500/result.json"
  ).then((response) => response.json());
  // .then((data) => data)
  // .then((data) => console.log(data.length));

  for (const circle of data.map((circle) => [
    circle[0] * 2 + 100,
    circle[1] * 2 + 150,
    circle[2] * 2,
    circle[3],
  ])) {
    ctx.beginPath();
    ctx.fillStyle = `rgba(${circle[3][0]}, ${circle[3][1]}, ${circle[3][2]})`;
    ctx.arc(circle[0], circle[1], circle[2], 0, 2 * Math.PI);
    ctx.fill();
  }
}

getData();
